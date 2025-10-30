/** @flow */
import * as React from 'react';
import LogView from './LogView';
import './LogChooser.css';

export default class FileChooser extends React.PureComponent {
  constructor(props, context) {
    super(props, context);

    this.state = {
      errMsg: null,
      data: null,
    };

    this._logFileChange = this._logFileChange.bind(this);
  }

  render() {
    if (this.state.data === null) {
      var msg = <div></div>;
      if (this.state.errMsg !== null) {
        msg = <div>Error: {this.state.errMsg}</div>;
      }

      // TODO: better landing screen (center it, format error, ... :D)
      // TODO: https://react-bootstrap.github.io/components/alerts
      return (
        <div>
          <h3>Pick your poison</h3>
          <input type="file" onChange={this._logFileChange} />
          {msg}
          {this.renderAPIExplainer()}
        </div>
      );
    } else {
      return (
        <LogView
          ref={(logview) => {
            window.logview = logview;
          }}
          data={this.state.data}
        />
      );
    }
  }

  _logFileChange(event) {
    var file = event.target.files[0];
    if (!file) {
      return;
    }

    document.title = `${file.name} – Cycle Debug Log Viewer`;
    var reader = new FileReader();
    reader.onload = e => {
      var contents = e.target.result;
      try {
        var data = JSON.parse(contents);
        this.setState({
          errMsg: null,
          data: data,
        })
      } catch (e) {
        this.setState({
          errMsg: `${e}`,
          data: null,
        })
      }
    };

    reader.onerror = e => {
      this.setState({
        errMsg: `Failed to load file ${file.name}. Error: ${e}`,
        data: null,
      })
    };

    reader.readAsText(file);
  }

  renderAPIExplainer() {
    return (
      <details className="api-explainer">
        <summary>How to programmatically control CDLV?</summary>
        <p>
          To allow semi-automated operation of CDL Viewer, we expose two
          JavaScript functions on the <code>window</code> object. Both functions
          return Promises that resolve to undefined upon completion.
        </p>
        <p>
          The first function is <code>load_data(parsed_json)</code> which feeds
          given object as the file to CDLV.
        </p>
        <p>
          The second function is{" "}
          <code>goto_cycle(cycle, mem_addr = undefined, fallible = false)</code>
          , which as soon as CDLV is ready scrolls it to given cycle and memory
          address (or first event at cycle if address is not given).{" "}
          <code>cycle</code> must be a number and <code>mem_addr</code> (if set)
          also must be a number (FYI <code>0x</code> prefix exists in JS and
          likely accomplishes what you need).
        </p>
        <p>
          By default, providing cycle or address that does not exist in the log
          only causes a warning, but setting the third argument to true will
          instead make the promise reject. This can be useful if you are
          handling promise rejection, but unhandled rejected promises can cause
          React to give up and replace your hard-earned CDL with a stack trace
          message.
        </p>
        <p>
          <strong>Example code:</strong> If you are serving a CDL log file{" "}
          <code>http://localhost:8000/cpb.json</code> with permissive CORS headers
          (such as <code>Access-Control-Allow-Origin: *</code>), you can load it
          through your browser's developer console like this:
        </p>
        <pre>
          {"fetch('http://localhost:8000/cbp.json')\n"}
          {"  .then(r => r.json())\n"}
          {"  .then(j => window.load_data(j))\n"}
          {"  .then(_ => window.goto_cycle(84455, 0x000000dc));\n"}
        </pre>
      </details>
    );
  }
}
