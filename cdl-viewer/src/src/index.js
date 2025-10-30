/** @flow */
import React from 'react';
import ReactDOM from 'react-dom';
import LogChooser from './LogChooser';
import 'bootstrap/dist/css/bootstrap.css';
import './index.css';
import 'bootstrap/dist/js/bootstrap.bundle.min';

ReactDOM.render(
  <React.StrictMode>
    <LogChooser
      ref={(chooser) => {
        window.chooser = chooser;
      }}
    />
  </React.StrictMode>,
  document.getElementById('root')
);

// idea for using ref to reach components taken from https://mtm.dev/update-component-state-outside-react/
function once_object_passes_test(
  object,
  test,
  action_to_take,
  fallible = false
) {
  return new Promise((resolve, reject) => {
    let interval;
    interval = setInterval(() => {
      if (test(object)) {
        clearInterval(interval);
        try {
          resolve(action_to_take(object));
        } catch (err) {
          if (fallible) {
            reject(err);
          } else {
            console.warn(err);
          }
        }
      }
    }, 10);
  });
}
window.load_data = (parsed_cdl_json) => (
  once_object_passes_test(
    window,
    (wndw) => wndw.chooser,
    (wndw) => {
      wndw.chooser.setState({ errMsg: null, data: parsed_cdl_json });
    }
  )
);
window.goto_cycle = (cycle, mem_addr, fallible = false) => (
  once_object_passes_test(
    window,
    (wndw) => wndw.logview && wndw.logview.state && wndw.logview.state.navIdx,
    (wndw) => {
      const navIdx = wndw.logview.state.navIdx;
      const cycleInfo = navIdx.cycle[cycle];
      if (cycleInfo === undefined) {
        throw new Error(`cycle ${cycle} has no data in loaded log`);
      }
      const col = cycleInfo.column + 1;
      let row = cycleInfo.eventsAtRow[0] + 1;
      if (mem_addr !== undefined) {
        const addressKey = mem_addr.toString(16).padStart(8, "0");
        const addressInfo = navIdx.address[addressKey];
        if (addressInfo === undefined) {
          wndw.logview._selectCell({ col, row });
          throw new Error(`address ${addressKey} has no data in loaded log`);
        }
        row = addressInfo.row + 1;
      }
      wndw.logview._selectCell({ col, row });
    },
    fallible
  )
)

window.load_logs = (logs_data, fallible = false) => (
    once_object_passes_test(
        window,
        (wndw) => wndw.logview && wndw.logview.state,
        (wndw) => wndw.logview._plainLogsProcess(logs_data),
        fallible
    )
)

window.onload = () => {
    const params = new Proxy(new URLSearchParams(window.location.search), {
        get: (searchParams, prop) => searchParams.get(prop),
    });
    if (params.src) {
        let promise =  fetch(params.src)
            .then(r => r.json())
            .then(j => window.load_data(j));
        if (params.log_src) {
            let logs = fetch(params.log_src)
                .then(r => {
                    if (!r.ok) throw new Error(`HTTP error ${r.status}`);
                    // return promise.then(_ => window.load_logs(r.body, false))
                    promise.then(_ => once_object_passes_test(
                        window,
                        (wndw) => wndw.logview && wndw.logview.state,
                        (wndw) => {
                            wndw.logview.setState({
                                plainLogsErrMsg: "Loading logs from URL...",
                                logsVisible: true,
                            });
                    }));
                    return r.blob();
                })
                .then(
                    // _ => _,
                    rb => promise.then(_ =>
                        window.load_logs(rb, true)
                    ),
                    fail => once_object_passes_test(
                        window,
                        (wndw) => wndw.logview && wndw.logview.state,
                        (wndw) => wndw.logview.setState({plainLogsErrMsg: fail.toString()}),
                        false,
                    )
                );
            if (params.cycle) {
                logs.then(_ =>
                        once_object_passes_test(
                            window,
                            (wndw) => wndw.logview && wndw.logview.state && wndw.logview.state.plainLogsData,
                            (wndw) => {
                                window.logview._scrollPlainLogsToCycle(params.cycle);
                            },
                            false,
                        )
                );
            }
        }
        if (params.cycle) {
            promise.then(_ => window.goto_cycle(parseInt(params.cycle), params.mem_addr || undefined)); // params return null
        }
    }
};
