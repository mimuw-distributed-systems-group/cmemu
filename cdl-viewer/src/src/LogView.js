/** @flow */
import * as React from 'react';
import { List, MultiGrid, AutoSizer } from 'react-virtualized';
import './LogView.css';
import { HotKeys } from "react-hotkeys";
import ansicolor from 'ansicolor';
// Note: using 0.6.22, since 0.7.0 doesn't work: https://github.com/nextapps-de/flexsearch/issues/72
import FlexSearch from 'flexsearch';

const DEBUG_PERF_LOG = true;


export default class LogView extends React.PureComponent {
  constructor(props, context) {
    super(props, context);

    this.state = {
      scrollToColumn: 0,
      scrollToRow: 0,
      sideMenuVisible: true,
      logsVisible: false,
      lastStatusMsg: null,
      searchCache: null,
      navIdx: null,
      searchIdx: null,
      selectionLocked: false,
      plainLogsErrMsg: null,
      plainLogsData: [],
      plainLogsFilteredData: [],
      plainLogsSearchRegex: "",
      rowFixedHalfword: localStorage.getItem('cdlv.rowFixedHalfword') === 'true',
      viewShowInstrWidths: localStorage.getItem('cdlv.viewShowInstrWidths') === 'true',
      scrollToAlignment: localStorage.getItem('cdlv.scrollToAlignment'),
    };

    this.data = props.data;
    // TODO: should it be here? -- only analysis needed for initial rendering
    this._analyse_memory();
    this.plainLogListRef = React.createRef();
    this._inputs = {};
    this._columnCount = this.data.cycles_desc.length + 1;
    this._rowCount = this.data.mem_instr_desc.length + 1;

    for (const functionName of [
      "_cellRenderer",
      "_toggleSideMenu",
      "_toggleLogs",
      "_onScrollToCycleChange",
      "_onScrollToAddressChange",
      "_onSearchChange",
      "_onSearchKeyDown",
      "_iterateColumnSelection",
      "_iterateRowSelection",
      "_plainLogsRenderItem",
      "_plainLogsHelpAlert",
      "_onPlainLogsFileChange",
      "_onPlainLogsSearchChange",
      "_scrollPlainLogsToCycle",
      "_onViewShowInstrWidthChanged",
      "_onRowFixedHalfwordChanged",
      "_onScrollToAlignmentChanged",
    ]) {
      this[functionName] = this[functionName].bind(this);
    }
    // TODO: use CellMeasurerCache if there's a problem with cell sizes
    // TODO: split.js for side menu
    // TODO: toggle horizontally side bar (animation?)
    // TODO: consider disable hover option & select cell by click
    // TODO: better search (filters - input text and/or checkboxes, maybe different tokenizers for various content, more logic in general)

    this._defineHotKeys();
  }

  _analyse_memory(recursed) {
    // This is computed in the constructor...
    let addresses = this.data.mem_instr_desc.map(d => ({addr: parseInt(d.addr, 16), wide: d.instr_len === 2}));
    const sentinel = {addr: 0, wide: false};
    this.mem_row_info = addresses.map(({addr, wide}, index) => {
      let next = addresses[index + 1] || sentinel;
      let addr_int = addr;
      let aligned = (addr & 3) === 0;
      let halfword = next.addr === addr + 2;
      let discontinuous = next.addr !== addr + 2 && next.addr !== addr + 4;
      let exec_spans = wide && halfword;
      let fetch_spans = aligned && halfword;
      return {addr_int, aligned, halfword, discontinuous, exec_spans, fetch_spans};
    })

    // Fix non-continuous rows and re-run analysis
    if (recursed === undefined && this.state.rowFixedHalfword) {
      this.data.mem_instr_desc = this.data.mem_instr_desc.flatMap((el, idx) => {
        let {addr_int, discontinuous, exec_spans, fetch_spans, halfword} = this.mem_row_info[idx];
        if (!halfword && (!discontinuous || fetch_spans || exec_spans)) {
          let empty_row = {synthetic: true, addr: (addr_int + 2).toString(16).padStart(8, '0')};
          return [el, empty_row];
        }
        return [el];
      })
      this._analyse_memory(true);
    }
  }

  _force_reload() {
    // FIXME: This is a hacky way to rebuild everything with a different rows layout.
    let data = this.data;
    // Undo our shenanigans
    data.mem_instr_desc = data.mem_instr_desc.filter(el => el.synthetic === undefined);

    let addr_int = this.state.selectedRowIndex ? this.mem_row_info[this.state.selectedRowIndex].addr_int : undefined;
    let cycle = this.state.selectedColumnIndex ? data.cycles_desc[this.state.selectedColumnIndex].cycle_no : undefined;
    window.load_data(null); // no change otherwise?
    window.load_data(data);
    if (cycle !== undefined) {
      window.goto_cycle(cycle, addr_int);
    }
  }

  componentDidMount() {
    asyncLogTimeOf("build navigation index", () => {
      this.setState({ navIdx: this._buildNavigationIndex(this.data) })
    });
    asyncLogTimeOf("build search index", () => {
      this.setState({ searchIdx: this._buildSearchIndex(this.data) })
    });
  }

  _defineHotKeys() {
    // Note: we don't want to support arrows: the grid handles the events anyway (causes scroll) and provides an extra lag
    this.KEY_MAP = {
      TOGGLE_SIDE_MENU: {
        name: "Toggle side menu",
        sequence: "m",
      },
      FOCUS_SCROLL_TO_CYCLE: {
        name: "Go to cycle",
        sequence: "c",
      },
      FOCUS_SCROLL_TO_ADDRESS: {
        name: "Go to address",
        sequence: "a",
      },
      FOCUS_SEARCH: {
        name: "Search",
        sequences: ["s", "/"],
      },
      SELECTION_UP: {
        name: "Move cell selection",
        sequence: "k",
      },
      SELECTION_DOWN: {
        name: "Move cell selection",
        sequence: "j",
      },
      SELECTION_LEFT: {
        name: "Move cell selection",
        sequence: "h",
      },
      SELECTION_RIGTH: {
        name: "Move cell selection",
        sequence: "l",
      },
      REMOVE_SELECTION: {
        name: "Remove cell selection",
        sequence: "escape",
      },
      SHOW_HELP: {
        name: "Show keyboard shortcuts",
        sequence: "shift+?",
      },
    };

    // inputs get created later, so we need a getter
    var createFocusInput = (get_input) => (() => {
      this.setState({ sideMenuVisible: true });
      // hack: if we focus the input immediately, the input handles the key event, too
      setTimeout(() => get_input().focus(), 50);
    });

    var createMoveSelection = (dc, dr) => (() => {
      if (this.state.selectionLocked) {
        var newCol = this.state.selectedColumnIndex + dc;
        var newRow = this.state.selectedRowIndex + dr;
        if (0 <= newCol && newCol < this._columnCount && 0 <= newRow && newRow < this._rowCount) {
          this._selectCell({
            col: newCol,
            row: newRow,
          });
        }
      }
    });

    var removeSelection = () => {
      this.setState({ selectionLocked: false });
    };

    var showHelp = () => {
      var desc = Object.entries(this.KEY_MAP).map(
        ([key, { name, sequence, sequences }]) =>
          (`${sequence || sequences.join(" or ")}: ${name}`)
      ).join("\n");
      var msg = "Note: holding keys lags the react-hotkey for a while\n" +
        "Note: you probably want to disable vimium and similar extensions\n" +
        "Note: arrows are not supported since it causes problems with MultiGrid\n" +
        "\n" + desc;
      alert(msg);
    };

    this.KEY_HANDLERS = {
      REMOVE_SELECTION: removeSelection,
      SELECTION_UP: createMoveSelection(0, -1),
      SELECTION_DOWN: createMoveSelection(0, 1),
      SELECTION_LEFT: createMoveSelection(-1, 0),
      SELECTION_RIGTH: createMoveSelection(1, 0),
      TOGGLE_SIDE_MENU: this._toggleSideMenu,
      FOCUS_SCROLL_TO_CYCLE: createFocusInput(() => this._inputs['Scroll to cycle']),
      FOCUS_SCROLL_TO_ADDRESS: createFocusInput(() => this._inputs['Scroll to address']),
      FOCUS_SEARCH: createFocusInput(() => this._inputs['Search']),
      SHOW_HELP: showHelp,
    };
  }

  render() {
    var grid = (
      <AutoSizer>
        {({ width, height }) => (
          <MultiGrid
            {...this.state}
            cellRenderer={this._cellRenderer}
            // TODO: following line does not work. MultiGrid does not recalculate size properly. (https://github.com/bvaughn/react-virtualized/issues/1004)
            // columnWidth={({ index }) => index === 0 ? 150 + (this.state.viewShowInstrWidths ? 20 : 0) : 80}
            // XXX: Those values were changed to 100, because with very large tables, react-virtualized create very large
            //   virtual vieport and puts 1e7-magnitude pixel offsets. Unfortunatelly, while JS is defined to work on doubles,
            //   setting `element.style['left'] = "100000001px"` does clipping to something that either is truncated to float32
            //   or just only 5 digits are retained of the scientific representation. 100 here works better than 64.
            //   Reproduction:   https://jsfiddle.net/L2gpvmf3/2/ -> check whether the printed values match the input,
            //   and whether there is a single pixel-wide black line visible when scrolled to the right.
            //   As of 11/22 only Safari renders that properly. Chrome -- only when directly from HTML/CSS. Firefox -- not at all.
            columnWidth={({ index }) => index === 0 ? 200 : 100}
            columnCount={this._columnCount}
            enableFixedColumnScroll
            enableFixedRowScroll
            fixedColumnCount={1}
            fixedRowCount={1}
            height={height}
            rowHeight={70}
            rowCount={this._rowCount}
            style={STYLE}
            width={width}
            hideTopRightGridScrollbar
            hideBottomLeftGridScrollbar
          />
        )}
      </AutoSizer>
    );

    // always render side menu, so dom remembers collapsable section state
    return (
      <HotKeys className="row flex-fill" keyMap={this.KEY_MAP} handlers={this.KEY_HANDLERS}>
        <div className={this.state.sideMenuVisible ? "col-xl-2 col-sm-3 side-menu-panel" : "d-none"}>
          {this._renderSideMenu()}
        </div>
        <div className="col"> {/* TODO: refactor side menus */}
          {grid}
        </div>
        <div className={this.state.logsVisible ? `col-xl-${localStorage.getItem('cdlv.plainLogsWidth') || "3"} col-sm-3 side-menu-panel` : "d-none"} style={{padding: 0, overflow: "hidden"}}>
          {this._renderLogs()}
        </div>
      </HotKeys>
    );
  }

  scrollMainViewToCycle(cycle) {
    if (this.state.navIdx.cycle[cycle + 1] !== undefined) {
      this._iterateColumnSelection(this.state.navIdx.cycle[cycle + 1].column);
    }
  }

  _renderSideMenu() {
    var navSection = this._createCollapsableSection({
      sectionId: 'navigation-section',
      expanded: true,
      header: <h4>Navigation</h4>,
      body: (<>
        {this._createLabeledInput({
          label: 'Scroll to cycle',
          icon: WATCH_ICON,
          placeholder: 'cycle_no[,event_no]',
          onChangeHandler: this._onScrollToCycleChange,
          enabledIf: this.state.navIdx !== null,
        })}
        {this._createLabeledInput({
          label: 'Scroll to address',
          icon: CODE_SLASH_ICON,
          placeholder: 'address_hex[,event_no]',
          onChangeHandler: this._onScrollToAddressChange,
          enabledIf: this.state.navIdx !== null,
        })}
        {this._createLabeledInput({
          label: 'Search',
          icon: SEARCH_ICON,
          placeholder: '(͠≖ ͜ʖ͠≖)',
          onChangeHandler: this._onSearchChange,
          onKeyDownHandler: this._onSearchKeyDown,
          enabledIf: this.state.navIdx !== null && this.state.searchIdx !== null,
        })}
        <div>
          <span className="font-weight-bold">Last status update</span> <br />
          {this.state.lastStatusMsg || "(none)"}
        </div>
      </>),
    });

    return (<>
      {navSection}
      {this._renderSideMenuView()}
      {this._renderSideMenuMetadata()}
      {this._renderSideMenuCycleInfo()}
      {this._renderSideMenuLegend()}
      <p className="small font-italic text-center">
        Press <span className="font-weight-bold">?</span> for shortcuts info
      </p>
    </>);
  }

  _renderLogs() {
    const time = (new Date()).toLocaleTimeString();
    let statusMsg;
    if (this.state.plainLogsErrMsg !== null) {
      statusMsg = <p>[{time}] {this.state.plainLogsErrMsg}</p>;
    } else if (this.state.plainLogsData.length < 1) {
      // eslint-disable-next-line jsx-a11y/anchor-is-valid
      statusMsg = <p>[{time}] Log file not loaded. <a href="#" onClickCapture={this._plainLogsHelpAlert}>?</a></p>;
    } else if (this.state.plainLogsSearchRegex !== "") {
      statusMsg = <p>[{time}] Showing {this.state.plainLogsFilteredData.length} of {this.state.plainLogsData.length} rows</p>;
    } else {
      statusMsg = <p>[{time}] Showing {this.state.plainLogsData.length} rows</p>;
    }
    const searchBoxHeight = 34, searchBoxVPadding = 5, rowHeight = 24;
    return (<>
      <AutoSizer>
        {({ height, width }) => (<>
          <div className="plain-log-control" style={{width: width, paddingTop: searchBoxVPadding, paddingBottom: searchBoxVPadding}}>
            <label htmlFor="plain-log-upload" title="Load plain text log file">{FILE_EARMARK_ARROW_UP_ICON}</label>
            <input id="plain-log-upload" type="file" style={{display: "none"}} onChange={this._onPlainLogsFileChange} />
            <input style={{height: searchBoxHeight}} type="search" placeholder="^.*$" onChange={this._onPlainLogsSearchChange} />
          </div>
          <div className="plain-log-msg" style={{height: rowHeight, width: width}}>{statusMsg}</div>
          <List
            {...this.state}
            ref={this.plainLogListRef}
            className="logList"
            height={height - searchBoxHeight - rowHeight - 2 * searchBoxVPadding}
            width={width}
            rowRenderer={this._plainLogsRenderItem}
            rowCount={this.state.plainLogsFilteredData.length}
            rowHeight={rowHeight}
            overscanRowCount={50}
            scrollToAlignment="center"
          />
        </>)}
      </AutoSizer>
    </>);
  }

  _renderSideMenuView() {
    return this._createCollapsableSection({
      sectionId: 'view-section',
      expanded: false,
      header: <h4>View</h4>,
      stickyHeader: true,
      noPadding: false,
      body: (<>
        {this._createLabeledCheckbox({
          label: "Show instruction widths",
          checked: () => this.state.viewShowInstrWidths,
          onChangeHandler: this._onViewShowInstrWidthChanged,
        })}
        {this._createLabeledCheckbox({
          label: "Use constant halfword-wide rows (forced refresh)",
          checked: () => this.state.rowFixedHalfword,
          onChangeHandler: this._onRowFixedHalfwordChanged,
        })}
        {this._createLabeledSelect({
          label: "Jumping method",
          selected: () => this.state.scrollToAlignment,
          onChangeHandler: this._onScrollToAlignmentChanged,
          options: {
            "auto": "scroll least needed amount",
            "start": "align to top-left",
            "center": "align to center",
            "end": "align to bottom-right",
          },
        })}
      </>),
    });
  }

  _renderSideMenuMetadata() {
    if (this.data.metadata && Object.keys(this.data.metadata).length > 0) {
      return this._createCollapsableSection({
        sectionId: 'metadata-section',
        expanded: false,
        header: <h4>Metadata</h4>,
        stickyHeader: true,
        noPadding: true,
        body: (<dl>
          {Object.keys(this.data.metadata).sort().map(key => <span key={key}>
            <dt>{key}</dt>
            <dd>{this.data.metadata[key]}</dd>
          </span>)}
        </dl>),
      });
    } else {
      return <></>;
    }
  }

  _renderSideMenuCycleInfo() {
    var precCycleDesc = this.data.cycles_desc[Math.max(0, this.state.selectedColumnIndex - 2) || 0];
    var cycleDesc = this.data.cycles_desc[Math.max(0, this.state.selectedColumnIndex - 1) || 0];

    var formatRegister = n => {
      var precVal = precCycleDesc.core.register_bank[n];
      var curVal = cycleDesc.core.register_bank[n];
      return (
        <div className={`d-flex mr-2 ${precVal !== curVal ? "value-changed" : ""}`} key={n}>
          <div className="mr-auto">r{n}:</div>
          <div className="ml-1">{curVal}</div>
        </div>
      );
    };

    var renderValue = (header, propGetter) => {
      var precVal = propGetter(precCycleDesc);
      var curVal = propGetter(cycleDesc);
      return (
        <p className={`${precVal !== curVal ? "value-changed" : ""}`}>
          <span className="font-weight-bold">{header}</span>
          <br />
          <span className="text-monospace">{curVal}</span>
        </p>
      );
    };

    // TODO: set css height, so registers can wrap into multiple columns (maybe just use col-sm?)
    var coreSection = this._createCollapsableSection({
      sectionId: 'cycle-core-info-section',
      expanded: false,
      header: <h6>Core</h6>,
      body: (<>
        {renderValue("Running mode", desc => desc.core.running_mode)}
        {renderValue("XPSR", desc => desc.core.xpsr)}
        {renderValue("CONTROL", desc => desc.core.control)}
        {renderValue("Stack pointers", desc => desc.core.stack_pointers)}
        <div>
          <span className="font-weight-bold">Registers</span>
          <div id="core-registers-container" className="text-monospace d-flex flex-wrap flex-column">
            {range(0, 16).map(formatRegister)}
          </div>
        </div>
      </>),
    });

    var dbusSection = this._createCollapsableSection({
      sectionId: 'cycle-dbus-info-section',
      expanded: false,
      header: <h6>Data Bus</h6>,
      body: (<>
        {renderValue("Request", desc => desc.dbus.request)}
        {renderValue("Response", desc => desc.dbus.response)}
        {renderValue("Data", desc => desc.dbus.data)}
      </>),
    });

    var dwtSection = this._createCollapsableSection({
      sectionId: 'cycle-dwt-info-section',
      expanded: false,
      header: <h6>DWT</h6>,
      body: (<>
        {renderValue("Cycle counter", desc => desc.dwt.cyccnt)}
        {renderValue("CPI counter", desc => desc.dwt.cpicnt)}
        {renderValue("LSU counter", desc => desc.dwt.lsucnt)}
        {renderValue("Fold counter", desc => desc.dwt.foldcnt)}
      </>),
    });

    // TODO: useful entries should be moved to a dedicated field
    var freeSection = this._createCollapsableSection({
      sectionId: 'cycle-free-status-section',
      expanded: false,
      header: <h6>Free status</h6>,
      body: (<>
        {
          Object.keys((cycleDesc.free_status || {})).sort().map(field =>
              renderValue(field, desc => (desc.free_status || {})[field])
        )}
      </>),
    });

    return this._createCollapsableSection({
      sectionId: 'cycle-info-section',
      expanded: false,
      header: <h4>Cycle #{cycleDesc.cycle_no}</h4>,
      stickyHeader: true,
      noPadding: true,
      body: (<>
        {coreSection}
        {dbusSection}
        {dwtSection}
        {freeSection}
      </>),
    });
  }

  _renderSideMenuLegend() {
    const phases = this._createCollapsableSection({
      sectionId: 'legend-phases-section',
      expanded: false,
      header: <h6>Pipeline phases</h6>,
      body: (<dl>
        <dt>F</dt><dd>Fetch</dd>
        <dt>D</dt><dd>Decode</dd>
        <dt>X</dt><dd>eXecute</dd>
        <dt>V</dt><dd>Vector fetch</dd>
        <dt>L</dt><dd>Literal load</dd>
      </dl>),
    });
    const suffixes = this._createCollapsableSection({
      sectionId: 'legend-suffixes-section',
      expanded: false,
      header: <h6>Suffixes</h6>,
      body: (<><dl>
        <dt>:IA</dt><dd>Instruction bus<sup><a title="More about bus utilization" href="#legend-bus-explanation">[1]</a></sup>, Address phase</dd>
        <dt>:ID</dt><dd>Instruction bus<sup><a title="More about bus utilization" href="#legend-bus-explanation">[1]</a></sup>, Data phase</dd>
        <dt>:DA</dt><dd>Data bus<sup><a title="More about bus utilization" href="#legend-bus-explanation">[1]</a></sup>, Address phase</dd>
        <dt>:DD</dt><dd>Data bus<sup><a title="More about bus utilization" href="#legend-bus-explanation">[1]</a></sup>, Data phase</dd>
        <dt>+</dt><dd>specified bus is in wait state</dd>
        <dt>-</dt><dd>specified bus did not receive a grant from BusMatrix</dd>
        <dt>:R</dt><dd><details>
          <summary>Fetch Registration occured</summary>
          <span className="details-content">
            Due to strict timing constraints on the system bus, fetching had to
            be delayed to the next cycle.
          </span>
        </details></dd>
        <dt>(P)</dt><dd><details>
          <summary>Pipelined</summary>
          <span className="details-content">
            An instruction is denoted as pipelined when it enters execution
            phase before the previous instruction has finished executing.
            It may happen in two situations:
            <ol>
              <li>There are two consecutive memory access instructions, and the
                address phase of the latter one occurs at the same time as the
                former's data phase.</li>
              <li>When <tt>xMULL</tt> instruction is followed
                with <tt>MLA</tt> instruction and certain register dependency
                occurs between these two.</li>
            </ol>
          </span>
        </details></dd>
        <dt>(S)</dt><dd><details>
          <summary>Skipped</summary>
          <span className="details-content">
            Execute phase of an instruction is skipped when its condition was
            not fulfilled. This can happen to conditional branch instructions
            and instructions in <tt>IT</tt> block. The instruction becomes
            effectively equivalent to a <tt>NOP</tt> instruction, however branch
            speculation might still affect Fetch state.
          </span>
        </details></dd>
        <dt>(F)</dt><dd><details>
          <summary>Folded</summary>
          <span className="details-content">
            Under certain circumstances, <tt>IT</tt> instruction may be folded,
            independently in Decode and Execute phase. The folded Execution phase
            occurs simultaneously with the previous instruction's analogous phase.
            The folded Decode phase occurs when previous instruction's has
            been decoded and not moved to Execute phase yet.
          </span>
        </details></dd>
      </dl>
        <span id="legend-bus-explanation" className="details-content">
          <strong><sup>[1]</sup> More about bus utilization: </strong>
          Instruction Bus is controlled by Fetch (phases: <tt>F</tt>, <tt>V</tt>,
          and indirectly interrupts: <tt>STK</tt>/<tt>UNSTK</tt>).
          Data Bus is controlled by LSU and used by Execute (phase <tt>X</tt>)
          and interrupts (phases <tt>STK</tt>/<tt>UNSTK</tt>).
        </span>
      </>),
    });
    const annotations = this._createCollapsableSection({
      sectionId: 'legend-annotations-section',
      expanded: false,
      header: <h6>Cycle annotations</h6>,
      stickyHeader: true,
      noPadding: true,
      body: (<dl>
        <dt>CPI</dt>
        <dd><tt>CPICNT</tt> increased</dd>
        <dt>LSU</dt>
        <dd><tt>LSUCNT</tt> increased</dd>
        <dt>FOLD</dt>
        <dd><tt>FOLDCNT</tt> increased</dd>
        <dt>STK</dt>
        <dd>stacking in progress (interrupt entry)</dd>
        <dt>UNSTK</dt>
        <dd>unstacking in progress (interrupt exit)</dd>
      </dl>),
    });
    return this._createCollapsableSection({
      sectionId: 'legend-section',
      expanded: false,
      header: <h4>Legend</h4>,
      stickyHeader: true,
      noPadding: true,
      body: (<>
        <p>
          For richer explanations,
          see <em>Jamro, Gutowski, Kordalski</em>,
          section 5.2.2. Cycle Debug Logger.
        </p>
        {phases}
        {suffixes}
        {annotations}
      </>),
    });
  }

  _createCollapsableSection({ sectionId, expanded, header, body, stickyHeader, noPadding }) {
    // TODO: consider subsection as "--- strike-through header ---"
    return (<>
      <div className={`row flex-fill side-menu-header ${stickyHeader ? "sticky" : ""}`}>
        <a role="button" data-toggle="collapse"
          aria-expanded={String(expanded)} aria-controls={sectionId} href={"#" + sectionId}
        >
          {header}
        </a>
      </div>
      <div id={sectionId} className={`row flex-fill collapse ${expanded ? "show" : ""} side-menu-section`}>
        <div className={`col ${noPadding ? "" : "pt-3 pb-3"}`}>
          {body}
        </div>
      </div>
    </>);
  }


  // This is dumb, but ansicolors return css string, but react need objects
   _convertStylesStringToObject = stringStyles => typeof stringStyles === 'string' ? stringStyles
    .split(';')
    .reduce((acc, style) => {
      const colonPosition = style.indexOf(':')

      if (colonPosition === -1) {
        return acc
      }

      const
        camelCaseProperty = style
          .substr(0, colonPosition)
          .trim()
          .replace(/^-ms-/, 'ms-')
          .replace(/-./g, c => c.substr(1).toUpperCase()),
        value = style.substr(colonPosition + 1).trim()

      return value ? {...acc, [camelCaseProperty]: value} : acc
    }, {}) : {}

  _plainLogsRenderItem({ index, style, registerChild, key }) {
    const elem = this.state.plainLogsFilteredData[index];
    const module = elem.module;
    const moduleSeparator = "::";
    const moduleSplit = module.lastIndexOf(moduleSeparator);
    const lastModule = moduleSplit !== -1 ? module.substring(moduleSplit + moduleSeparator.length) : module;
    const cycleOfPreviousIndex = (this.state.plainLogsFilteredData[index - 1] || {}).cycle || -1;
    const isFirstOfCycle = (elem.cycle || -1) > cycleOfPreviousIndex;
    const innerCycleDisplay = elem.cycle !== undefined ? <><button className="link-button" onClick={() => this.scrollMainViewToCycle(elem.cycle)}>{elem.cycle}</button>{" "}</> : <></>;
    const cycleDisplay = isFirstOfCycle ? <strong>{innerCycleDisplay}</strong> : innerCycleDisplay;
    var className = "log-message";

    //  We have ints, but they have strings
    // eslint-disable-next-line
    if (elem.cycle == (this.data.cycles_desc[this.state.selectedColumnIndex-1]||{}).cycle_no) {
      className += " log-message-selected-darken";
    }
    return (<span ref={registerChild} key={key} style={style}>
      <div className={className}>
        {cycleDisplay}
        {LEVEL_SYMBOLS[elem.level]}{" "}
        <abbr title={module}>{lastModule}</abbr>{" "}
        {ansicolor.parse(elem.message).spans.map(s => {
          const style = this._convertStylesStringToObject(s.css);
          return <span style={style}>{s.text}</span>
          }
        )}
      </div>
    </span>);
  }

  _createLabeledInput({ label, icon, placeholder, onChangeHandler, onKeyDownHandler, enabledIf }) {
    return (
      <div className="mb-3">
        <label className="font-weight-bold">{label}</label>
        <div className="input-group">
          {
            icon !== undefined ?
              <div className="input-group-prepend">
                <span className="input-group-text">{icon}</span>
              </div>
              : null
          }
          <input type="text" className="form-control" placeholder={placeholder}
            onChange={onChangeHandler} onKeyDown={onKeyDownHandler} disabled={!enabledIf}
            ref={(input) => { this._inputs[label] = input; }} />
        </div>
      </div>
    );
  }

  _createLabeledCheckbox({ label, checked, onChangeHandler }) {
    return (
      <div className="mb-3">
        <div className="input-group">
          <label className="form-check-label">
            <input type="checkbox" className="form-check-input" onChange={onChangeHandler}
              checked={checked()} ref={(input) => { this._inputs[label] = input; }} />
            {label}
          </label>
        </div>
      </div>
    );
  }

  _createLabeledSelect({ label, options, selected, onChangeHandler }) {
    const optionElements = Object.entries(options).map(([value, description]) => (
      <option value={value} selected={selected() === value}>
        {description}
      </option>
    ));
    return (
      <div className="mb-3">
        <label className="font-weight-bold">{label}</label>
        <div className="input-group">
          <select onChange={onChangeHandler}>
            {optionElements}
          </select>
        </div>
      </div>
    );
  }



  _cellRenderer({ columnIndex, key, rowIndex, style }) {
    var body;
    var className = "cell";
    var onClick;
    var postBodyAnnotation;

    if (columnIndex === this.state.selectedColumnIndex || rowIndex === this.state.selectedRowIndex) {
      className += " cell-selected-darken";
    }
    if (this.state.selectionLocked
      && columnIndex === this.state.selectedColumnIndex && rowIndex === this.state.selectedRowIndex) {
      className += " cell-selected-framed";
    }

    var toggleSelectionOnThisCell = () => {
      if (this.state.selectionLocked
        && this.state.selectedColumnIndex === columnIndex
        && this.state.selectedRowIndex === rowIndex) {
        this.setState({
          selectionLocked: false,
        });
        // TODO: grid force update?
      } else {
        this.setState({
          selectionLocked: true,
          selectedColumnIndex: columnIndex,
          selectedRowIndex: rowIndex,
        });
        // TODO: grid force update?
      }
    };

    var onMouseOver = () => {
      if (!this.state.selectionLocked) {
        this.setState({
          selectedColumnIndex: columnIndex,
          selectedRowIndex: rowIndex,
        });
        // TODO: grid force update?
      }
    };

    const _row_classes = (mem_info, tags) => {
      var className = "";
      if (!mem_info.aligned) {
        className += " log-view--unaligned";
      }
      if (mem_info.discontinuous) {
        className += " log-view--discontinuous";
      }
      /* Cell spanning works by extending the background height x2,
      * while making empty cell below them. */
      if ((tags.includes("F") && mem_info.fetch_spans) ||
          ((tags.includes("D") || tags.includes("X")) && mem_info.exec_spans)) {
        className += " log-view--spanning";
      }
      return className
    }

    if (columnIndex === 0 && rowIndex === 0) {
      className += " log-view--corner";
      body = (<>
        <button onClick={this._toggleSideMenu}>{GEAR_ICON}</button>
        <button onClick={this._toggleLogs}>{LIST_TASK_ICON}</button>
      </>);
    } else if (rowIndex === 0) {
      let cycle = this.data.cycles_desc[columnIndex - 1];

      className += " log-view--top-header";
      onClick = () => {
        this._iterateColumnSelection(columnIndex);
        this._scrollPlainLogsToCycle(cycle.cycle_no);
      };
      body = cycle.cycle_no;

      var columnHeaders = [];
      if (cycle.core.stacking_mode) { columnHeaders.push(cycle.core.stacking_mode); }
      if (cycle.dwt.cpicnt_incremented === true) { columnHeaders.push("CPI"); }
      if (cycle.dwt.lsucnt_incremented === true) { columnHeaders.push("LSU"); }
      if (cycle.dwt.foldcnt_incremented === true) { columnHeaders.push("FOLD"); }
      postBodyAnnotation = columnHeaders.join(", ");
    } else if (columnIndex === 0) {
      var mem_instr = this.data.mem_instr_desc[rowIndex - 1];
      let mem_info = this.mem_row_info[rowIndex - 1];

      var rendered_instr = mem_instr.instr;
      if (typeof rendered_instr === 'string' && this.state.viewShowInstrWidths) {
        var suffix = ({ 1: ".n ", 2: ".w " })[mem_instr.instr_len] || ".err ";
        rendered_instr = rendered_instr.replace(" ", suffix);
      }

      className += " log-view--left-header";
      className += _row_classes(mem_info, ["X"]); // we span if instruction
      onClick = () => this._iterateRowSelection(rowIndex);

      body = <>
        {mem_instr.addr} <br />
        {rendered_instr}
      </>;

    } else {
      var addr = this.data.mem_instr_desc[rowIndex - 1].addr;
      let mem_info = this.mem_row_info[rowIndex - 1];
      let cycle = this.data.cycles_desc[columnIndex - 1];
      let cycle_no = cycle.cycle_no;

      var idx = `${addr},${cycle_no}`;
      var events = this.data.events[idx];
      var tags = [];
      if (events !== undefined) {
        // TODO: consider caching these values
        tags = events.map(line => line[0]);
        var colors = tags.map(tag => EVENT_COLORMAP[tag]);
        events = events.reduce((result, item) => <>{result}<br />{item}</>);
        this._ensureStyle(style, "--bg-col", colors[0]);
        this._ensureStyle(style, "--bg-img", `linear-gradient(${colors.join(", ")})`);
      } else if (cycle.core.stacking_mode) {
        this._ensureStyle(style, "--bg-col", EVENT_COLORMAP["STK/UNSTK"]);
      } else {
        this._ensureStyle(style, "--bg-col", EVENT_COLORMAP[""]);
        className += " cell-empty";
      }

      className += _row_classes(mem_info, tags);
      onClick = toggleSelectionOnThisCell;
      body = events;
    }

    return (
      <div className={className} key={key} style={style} onClick={onClick} onMouseOver={onMouseOver}>
        <span className="cell-inside">
          {body}
        </span>
        <span className="cell-inside cell-annotation">
          {postBodyAnnotation}
        </span>
      </div>
    );
  }

  _scrollPlainLogsToCycle(cycle) {
    const firstLine = this._plainLogsFindRowFirstItem(cycle);
    if (firstLine !== null) {
      this.plainLogListRef.current.scrollToRow(firstLine);
    }
  }

  _ensureStyle(style, prop, value) {
    if (style[prop] === undefined) {
      style[prop] = value;
    } else {
      console.assert(style[prop] === value);
    }
  }

  _toggleSideMenu() {
    this.setState({
      sideMenuVisible: !this.state.sideMenuVisible,
    })
  }

  _toggleLogs() {
    this.setState({
      logsVisible: !this.state.logsVisible,
    });
  }

  _SCROLL_TO_CYCLE_REGEX = /^\s*\d+\s*(,\s*(\d+\s*)?)?$/;
  _onScrollToCycleChange(event) {
    var value = event.target.value;
    var scrollToColumn, scrollToRow, lastStatusMsg;
    if (this._SCROLL_TO_CYCLE_REGEX.test(value)) {
      var [cycle, event_no] = value.split(",").map(no => parseInt(no || 0));
      event_no = Math.max(1, event_no || 0) - 1; // optional field
      var cycleInfo = this.state.navIdx.cycle[cycle];
      if (cycleInfo !== undefined) {
        event_no = Math.min(event_no, cycleInfo.eventsAtRow.length - 1);
        scrollToColumn = cycleInfo.column + 1;
        scrollToRow = cycleInfo.eventsAtRow[event_no] + 1;
        lastStatusMsg = `scroll(cycle: ${cycle}, \
          event: ${event_no + 1} of ${cycleInfo.eventsAtRow.length})`;
      } else {
        lastStatusMsg = `cycle #${cycle} not found`;
      }
    } else {
      lastStatusMsg = `invalid format`;
    }

    this._selectCell({ col: scrollToColumn, row: scrollToRow });
    this.setState({
      lastStatusMsg: lastStatusMsg,
    });
  }

  _SCROLL_TO_ADDRESS_REGEX = /^\s*[0-9a-fA-F]+\s*(,\s*(\d+\s*)?)?$/;
  _onScrollToAddressChange(event) {
    var value = event.target.value;
    var scrollToColumn, scrollToRow, lastStatusMsg;
    if (this._SCROLL_TO_ADDRESS_REGEX.test(value)) {
      var [address, event_no] = value.split(",");
      address = parseInt(address, 16).toString(16).padStart(8, '0');
      event_no = Math.max(1, parseInt(event_no || 0)) - 1; // optional field
      var addressInfo = this.state.navIdx.address[address];
      if (addressInfo !== undefined) {
        event_no = Math.min(event_no, addressInfo.eventsAtColumn.length - 1);
        scrollToColumn = addressInfo.eventsAtColumn[event_no] + 1;
        scrollToRow = addressInfo.row + 1;
        lastStatusMsg = `scroll(address: ${address}, \
          event: ${event_no + 1} of ${addressInfo.eventsAtColumn.length})`;
      } else {
        // TODO: consider: use closest/preceding address instead
        lastStatusMsg = `address ${address} not found`;
      }
    } else {
      lastStatusMsg = `invalid format`;
    }

    this._selectCell({ col: scrollToColumn, row: scrollToRow });
    this.setState({
      lastStatusMsg: lastStatusMsg,
    });
  }

  _selectCell({ col, row }) {
    this.setState({
      selectedColumnIndex: col,
      selectedRowIndex: row,
      scrollToColumn: col,
      scrollToRow: row,
      selectionLocked: true,
    })
  }

  _iterateColumnSelection(column) {
    var targetRowIdx = 0; // by default select the header
    if (!this.state.selectionLocked || this.state.selectedColumnIndex !== column) {
      // use the default
    } else {
      var cycle_no = this.data.cycles_desc[column - 1].cycle_no;
      var res = upperBoundValue(this.state.navIdx.cycle[cycle_no].eventsAtRow, this.state.selectedRowIndex - 1);
      if (res !== null) {
        targetRowIdx = res + 1;
      }
    }

    this._selectCell({
      col: column,
      row: targetRowIdx,
    })
  }

  _iterateRowSelection(row) {
    var targetColumnIdx = 0; // by default select the header
    if (!this.state.selectionLocked || this.state.selectedRowIndex !== row) {
      // use the default
    } else {
      var addr = this.data.mem_instr_desc[row - 1].addr;
      var res = upperBoundValue(this.state.navIdx.address[addr].eventsAtColumn, this.state.selectedColumnIndex - 1);
      if (res !== null) {
        targetColumnIdx = res + 1;
      }
    }

    this._selectCell({
      col: targetColumnIdx,
      row: row,
    })
  }

  _onPlainLogsFileChange(event) {
    const file = event.target.files[0];
    if (!file) return;
    this._plainLogsProcess(file);
  }

  // TODO: make sure that it works after changing 'quartz' to 'clock_tree'.
  async _plainLogsProcess(file) {
    function yieldCPU() {
      return new Promise((resolve, _reject) => setTimeout(resolve, 0));
    }
    function FileLineIterator(file) {
      let buffer = "";
      const chunkSize = 1024 * 1024;
      let lastStartPos = 0 - chunkSize;
      let bufferPos = 0;
      let terminated = false;
      return {
        async next() {
          if (terminated) {
            return { done: true };
          }
          let newlinePos;
          while ((newlinePos = buffer.indexOf("\n", bufferPos)) === -1) {
            lastStartPos += chunkSize;
            if (lastStartPos > file.size) {
              newlinePos = buffer.length;
              terminated = true;
              break;
            }
            buffer = buffer.slice(bufferPos) + await file.slice(lastStartPos, lastStartPos + chunkSize).text();
            bufferPos = 0;
          }
          const value = buffer.slice(bufferPos, newlinePos);
          bufferPos = newlinePos + 1;
          return { done: false, value };
        },
        [Symbol.asyncIterator]() {
          return this;
        }
      };
    }
    const processingStartTime = performance.now();
    const data = [];
    const expr = /^\s*(?<level>TRACE|DEBUG|ERROR|WARN|INFO)\s+(?<module>(?:[a-zA-Z0-9_]+::)*[a-zA-Z0-9_]+)\s*> (?<message>.*?)$/u;
    let lastCycle = undefined;
    let errMsg = null;
    const tickSuffix = " ================================"
    const tickPrefix = "================================ FAST_TICK: ";
    let lineCount = 0;
    let cycleLines = 0;
    let erroneousLines = 0;
    for await (const line of FileLineIterator(file)) {
      const match = expr.exec(line);
      if (match === null) {
        erroneousLines++;
        continue;
      }
      const level = match.groups.level, module = match.groups.module, message = match.groups.message.trim();
      const stripped_message = ansicolor.strip(message);
      if (level === "TRACE" && module === "cmemu_lib::component::clock_tree" && message.startsWith(tickPrefix) && message.endsWith(tickSuffix)) {
        cycleLines++;
        lastCycle = parseInt(message.substring(tickPrefix.length, message.length - tickSuffix.length));
      } else {
        data.push({ cycle: lastCycle, level, module, stripped_message, message });
      }
      lineCount++;
      if (lineCount % 12345 === 0) {
        this.setState({ plainLogsErrMsg: `Processing ${file.name}: ${lineCount} lines so far` });
        await yieldCPU();
      }
    }
    console.log("cycle tick lines:", cycleLines);
    console.log("erroneous lines:", erroneousLines);
    if (data.length < 1) {
      // eslint-disable-next-line jsx-a11y/anchor-is-valid
      errMsg = <>No valid lines found in file. <a href="#" onClickCapture={this._plainLogsHelpAlert}>?</a></>;
    }
    this.setState({ plainLogsData: data, plainLogsErrMsg: errMsg, plainLogsFilteredData: data });
    if (this.state.plainLogsErrMsg === null) {
      this._plainLogsSearch(this.state.plainLogsSearchRegex);
    }
    const processingEndTime = performance.now();
    console.log("plain log processing time:", processingEndTime - processingStartTime, "milliseconds")
  }

  _plainLogsHelpAlert() {
    const msg = (
      "To obtain plain logs, redirect standard error stream from the program to a file, while configuring logging via the RUST_LOG environment variable.\n"
      + "For details on how to configure RUST_LOG, see https://lib.rs/crates/env_logger\n"
      + "To be able to navigate the logs by cycle, remember to enable logging clock ticks by setting cmemu_lib::component::clock_tree=trace.\n"
      + "\n"
      + "Example command that collects CDL and logs:\n"
      + "RUST_LOG=cmemu_lib::component::core=trace,cmemu_lib::component::clock_tree=trace cargo run -p cmemu-flash-test -- cmemu-tests/tests/flash/instructions/control_flow/it_fold.tzst.0 --cycle-debug-log-file=it_fold_2.json 2> it_fold_2.log"
    );
    alert(msg);
  }

  _onSearchChange() {
    this.setState({
      lastStatusMsg: 'press <Enter> to search',
    })
  }

  _onPlainLogsSearchChange(event) {
    const newValue = event.target.value;
    this._plainLogsSearch(newValue);
  }

  _plainLogsSearch(regexString) {
    const stateChange = {};
    if (regexString === "") {
      stateChange.plainLogsSearchRegex = "";
      stateChange.plainLogsErrMsg = null;
      stateChange.plainLogsFilteredData = this.state.plainLogsData;
    } else {
      try {
        const expr = new RegExp(regexString, 'iu');
        stateChange.plainLogsSearchRegex = regexString;
        stateChange.plainLogsErrMsg = null;
        stateChange.plainLogsFilteredData = this.state.plainLogsData.filter(elem => expr.test(`${elem.level} ${elem.module} ${elem.stripped_message}`));
      } catch (e) {
        stateChange.plainLogsErrMsg = e;
      }
    }
    this.setState(stateChange);
  }

  _onSearchKeyDown(event) {
    if (event.key === 'Enter') {
      var query = event.target.value;
      if (this.state.searchCache === null || this.state.searchCache.query !== query) {
        this.setState({
          lastStatusMsg: (<>
            <div className="spinner-border spinner-border-sm mr-1" role="status" ></div>
            searching <span className="font-italic">{query}</span>...
          </>),
        })
        this._search(query).then(
          (results) => this._selectSearchResult({ query: query, freshResults: results })
        );
      } else {
        this._selectSearchResult({ query: query, stepResults: event.shiftKey ? -1 : 1 })
      }
    }
  }

  _onViewShowInstrWidthChanged(event) {
    let newValue = !this.state.viewShowInstrWidths;
    localStorage.setItem('cdlv.viewShowInstrWidths', newValue);
    this.setState({ viewShowInstrWidths: newValue });
  }

  _onRowFixedHalfwordChanged(event) {
    let newValue = !this.state.rowFixedHalfword;
    localStorage.setItem('cdlv.rowFixedHalfword', newValue);
    this.setState({rowFixedHalfword: newValue});
    this._force_reload();
  }

  _onScrollToAlignmentChanged(event) {
    let newValue = event.target.value;
    localStorage.setItem('cdlv.scrollToAlignment', newValue);
    this.setState({ scrollToAlignment: newValue });
  }

  async _search(query) {
    return {
      cycle: await this.state.searchIdx.cycle.search(query),
      memInstr: await this.state.searchIdx.memInstr.search(query),
      event: await this.state.searchIdx.event.search(query),
    };
  }

  _selectSearchResult({ freshResults, stepResults, query }) {
    console.assert(freshResults === undefined || stepResults === undefined);

    var sc;
    if (freshResults) {
      var resultsLength = (
        freshResults.cycle.length
        + freshResults.memInstr.length
        + freshResults.event.length
      );

      this.setState({
        searchCache: {
          query: query,
          results: freshResults,
          selectedIdx: 0,
          resultsLength: resultsLength,
        }
      })
    } else {
      sc = this.state.searchCache;
      sc.selectedIdx = (sc.selectedIdx + stepResults + sc.resultsLength) % sc.resultsLength;
      this.setState({
        searchCache: sc,
      })
    }

    sc = this.state.searchCache;
    if (sc.resultsLength === 0) {
      const NOT_FOUND_REACTIONS = ['乁(ᴗ ͜ʖ ᴗ)ㄏ', '( ཀ ʖ̯ ཀ)', '(ʘ言ʘ╬)', '(┛ಠДಠ)┛彡┻━┻'];
      this.setState({
        lastStatusMsg: <>Not found. <br /> {randomChoice(NOT_FOUND_REACTIONS)}</>
      })
    } else {
      var col, row, desc;
      var idx = sc.selectedIdx;
      if (idx < sc.results.cycle.length) {
        // cycle
        row = -1;
        col = sc.results.cycle[idx];
        desc = `cycle #${this.data.cycles_desc[col].cycle_no}`;
      } else {
        idx -= sc.results.cycle.length;
        if (idx < sc.results.memInstr.length) {
          // address
          row = sc.results.memInstr[idx];
          col = -1;
          desc = `address ${this.data.mem_instr_desc[row].addr}`;
        } else {
          // event
          idx -= sc.results.memInstr.length;
          var { addr, cycle_no } = keyToAddrCycle(sc.results.event[idx]);
          row = this.state.navIdx.address[addr].row;
          col = this.state.navIdx.cycle[cycle_no].column;
          desc = `event (addr: ${addr}, cycle #${cycle_no})`;
        }
      }
      const msg = (<>
        Result {sc.selectedIdx + 1} of {sc.resultsLength}: {desc}
        <br />
        <br />
        <small>{"Use [<Shift>+]<Enter> for previous / next result."}</small>
      </>);
      this._selectCell({ col: col + 1, row: row + 1 });
      this.setState({
        lastStatusMsg: msg,
      });
    }
  }

  _buildNavigationIndex(data) {
    var navIdx = {
      cycle: {},
      address: {},
    };

    data.cycles_desc.forEach((item, idx) => {
      navIdx.cycle[item.cycle_no] = {
        column: idx,
        eventsAtRow: [],
      };
    });

    data.mem_instr_desc.forEach((item, idx) => {
      navIdx.address[item.addr] = {
        row: idx,
        eventsAtColumn: [],
      }
    });

    Object.keys(data.events).forEach(key => {
      var { addr, cycle_no } = keyToAddrCycle(key);
      var addrInfo = navIdx.address[addr];
      var cycleInfo = navIdx.cycle[cycle_no];
      addrInfo.eventsAtColumn.push(cycleInfo.column);
      cycleInfo.eventsAtRow.push(addrInfo.row);
    });

    const cmpNums = (a, b) => a - b;
    Object.values(navIdx.cycle).forEach(info => info.eventsAtRow.sort(cmpNums));
    Object.values(navIdx.address).forEach(info => info.eventsAtColumn.sort(cmpNums));

    return navIdx;
  }

  _plainLogsFindRowFirstItem(cycle) {
    let lower = 0, upper = this.state.plainLogsFilteredData.length;
    while (lower < upper - 1) {
      const middle = Math.floor((lower + upper) / 2);
      const row = this.state.plainLogsFilteredData[middle];
      if (row.cycle !== undefined) {
        if (row.cycle > cycle) {
          upper = middle;
        } else if (row.cycle < cycle) {
          lower = middle + 1;
        } else /* row.cycle === cycle */{
          const prevRowCycle = middle - 1 < 0 ? row.cycle - 1 : this.state.plainLogsFilteredData[middle - 1].cycle;
          if (prevRowCycle === undefined || prevRowCycle < cycle) {
            return middle;
          } else /* prevRowCycle === cycle */ {
            upper = middle;
          }
        }
      } else {
        lower = middle + 1;
      }
    }
    if (lower < upper) {
      return lower;
    } else {
      return null;
    }
  }

  _buildSearchIndex(data) {
    var searchIdx = {};

    function flattenValueAsStr(val) {
      // ref: https://www.w3schools.com/js/js_datatypes.asp
      if (val === null || val === undefined || typeof (val) === "function" || typeof (val) === "boolean") {
        // nothing
        return "";
      } else if (typeof (val) === "object") {
        var ret = "";
        for (const prop in val) {
          if (val.hasOwnProperty(prop)) {
            ret += " " + flattenValueAsStr(val[prop]);
          }
        }
        return ret.trim();
      } else {
        return String(val);
      }
    }

    const settings = {
      tokenize: "full",
      encode: "icase",
      async: true,
    };

    searchIdx.cycle = new FlexSearch(settings);
    data.cycles_desc.forEach((item, idx) => {
      searchIdx.cycle.add(idx, flattenValueAsStr(item));
    })

    searchIdx.memInstr = new FlexSearch(settings);
    data.mem_instr_desc.forEach((item, idx) => {
      searchIdx.memInstr.add(idx, flattenValueAsStr(item));
    })

    // maybe different tokenizer for events?
    searchIdx.event = new FlexSearch(settings);
    Object.entries(data.events).forEach(([idx, item]) => {
      searchIdx.event.add(idx, flattenValueAsStr(item));
    })

    return searchIdx;
  }
}

// ----------------------------------------------------------------------------

const STYLE = {
  border: '1px solid #ddd',
};

const EVENT_COLORMAP = {
  "": 'var(--cell-empty-bg-col)',
  V: 'var(--cell-event-vector-fetch-bg-col)',
  F: 'var(--cell-event-fetch-bg-col)',
  D: 'var(--cell-event-decode-bg-col)',
  X: 'var(--cell-event-execute-bg-col)',
  L: 'var(--cell-event-literal-bg-col)',
  "STK/UNSTK": 'var(--cell-event-stacking-unstacking-bg-col)',
  undefined: 'var(--cell-event-undefined-bg-col)',
};

// Bootstrap icons: https://icons.getbootstrap.com/
const GEAR_ICON = (
  <svg xmlns="http://www.w3.org/2000/svg" width="2em" height="2em" fill="currentColor" className="bi bi-gear" viewBox="0 0 16 16">
    <path d="M8 4.754a3.246 3.246 0 1 0 0 6.492 3.246 3.246 0 0 0 0-6.492zM5.754 8a2.246 2.246 0 1 1 4.492 0 2.246 2.246 0 0 1-4.492 0z"/>
    <path d="M9.796 1.343c-.527-1.79-3.065-1.79-3.592 0l-.094.319a.873.873 0 0 1-1.255.52l-.292-.16c-1.64-.892-3.433.902-2.54 2.541l.159.292a.873.873 0 0 1-.52 1.255l-.319.094c-1.79.527-1.79 3.065 0 3.592l.319.094a.873.873 0 0 1 .52 1.255l-.16.292c-.892 1.64.901 3.434 2.541 2.54l.292-.159a.873.873 0 0 1 1.255.52l.094.319c.527 1.79 3.065 1.79 3.592 0l.094-.319a.873.873 0 0 1 1.255-.52l.292.16c1.64.893 3.434-.902 2.54-2.541l-.159-.292a.873.873 0 0 1 .52-1.255l.319-.094c1.79-.527 1.79-3.065 0-3.592l-.319-.094a.873.873 0 0 1-.52-1.255l.16-.292c.893-1.64-.902-3.433-2.541-2.54l-.292.159a.873.873 0 0 1-1.255-.52l-.094-.319zm-2.633.283c.246-.835 1.428-.835 1.674 0l.094.319a1.873 1.873 0 0 0 2.693 1.115l.291-.16c.764-.415 1.6.42 1.184 1.185l-.159.292a1.873 1.873 0 0 0 1.116 2.692l.318.094c.835.246.835 1.428 0 1.674l-.319.094a1.873 1.873 0 0 0-1.115 2.693l.16.291c.415.764-.42 1.6-1.185 1.184l-.291-.159a1.873 1.873 0 0 0-2.693 1.116l-.094.318c-.246.835-1.428.835-1.674 0l-.094-.319a1.873 1.873 0 0 0-2.692-1.115l-.292.16c-.764.415-1.6-.42-1.184-1.185l.159-.291A1.873 1.873 0 0 0 1.945 8.93l-.319-.094c-.835-.246-.835-1.428 0-1.674l.319-.094A1.873 1.873 0 0 0 3.06 4.377l-.16-.292c-.415-.764.42-1.6 1.185-1.184l.292.159a1.873 1.873 0 0 0 2.692-1.115l.094-.319z"/>
  </svg>
);

const LIST_TASK_ICON = (
  <svg xmlns="http://www.w3.org/2000/svg" width="2em" height="2em" fill="currentColor" className="bi bi-list-task" viewBox="0 0 16 16">
    <path fillRule="evenodd" d="M2 2.5a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h1a.5.5 0 0 0 .5-.5V3a.5.5 0 0 0-.5-.5H2zM3 3H2v1h1V3z"/>
    <path d="M5 3.5a.5.5 0 0 1 .5-.5h9a.5.5 0 0 1 0 1h-9a.5.5 0 0 1-.5-.5zM5.5 7a.5.5 0 0 0 0 1h9a.5.5 0 0 0 0-1h-9zm0 4a.5.5 0 0 0 0 1h9a.5.5 0 0 0 0-1h-9z"/>
    <path fillRule="evenodd" d="M1.5 7a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5H2a.5.5 0 0 1-.5-.5V7zM2 7h1v1H2V7zm0 3.5a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h1a.5.5 0 0 0 .5-.5v-1a.5.5 0 0 0-.5-.5H2zm1 .5H2v1h1v-1z"/>
  </svg>
);

const WATCH_ICON = (
  <svg className="bi bi-watch" width="1em" height="1em" viewBox="0 0 16 16" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
    <path fillRule="evenodd" d="M4 14.333v-1.86A5.985 5.985 0 0 1 2 8c0-1.777.772-3.374 2-4.472V1.667C4 .747 4.746 0 5.667 0h4.666C11.253 0 12 .746 12 1.667v1.86A5.985 5.985 0 0 1 14 8a5.985 5.985 0 0 1-2 4.472v1.861c0 .92-.746 1.667-1.667 1.667H5.667C4.747 16 4 15.254 4 14.333zM13 8A5 5 0 1 0 3 8a5 5 0 0 0 10 0z" />
    <rect width="1" height="2" x="13.5" y="7" rx=".5" />
    <path fillRule="evenodd" d="M8 4.5a.5.5 0 0 1 .5.5v3a.5.5 0 0 1-.5.5H6a.5.5 0 0 1 0-1h1.5V5a.5.5 0 0 1 .5-.5z" />
  </svg>
);

const CODE_SLASH_ICON = (
  <svg className="bi bi-code-slash" width="1em" height="1em" viewBox="0 0 16 16" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
    <path fillRule="evenodd" d="M4.854 4.146a.5.5 0 0 1 0 .708L1.707 8l3.147 3.146a.5.5 0 0 1-.708.708l-3.5-3.5a.5.5 0 0 1 0-.708l3.5-3.5a.5.5 0 0 1 .708 0zm6.292 0a.5.5 0 0 0 0 .708L14.293 8l-3.147 3.146a.5.5 0 0 0 .708.708l3.5-3.5a.5.5 0 0 0 0-.708l-3.5-3.5a.5.5 0 0 0-.708 0zm-.999-3.124a.5.5 0 0 1 .33.625l-4 13a.5.5 0 0 1-.955-.294l4-13a.5.5 0 0 1 .625-.33z" />
  </svg>
);

const SEARCH_ICON = (
  <svg className="bi bi-search" width="1em" height="1em" viewBox="0 0 16 16" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
    <path fillRule="evenodd" d="M10.442 10.442a1 1 0 0 1 1.415 0l3.85 3.85a1 1 0 0 1-1.414 1.415l-3.85-3.85a1 1 0 0 1 0-1.415z" />
    <path fillRule="evenodd" d="M6.5 12a5.5 5.5 0 1 0 0-11 5.5 5.5 0 0 0 0 11zM13 6.5a6.5 6.5 0 1 1-13 0 6.5 6.5 0 0 1 13 0z" />
  </svg>
);

const FILE_EARMARK_ARROW_UP_ICON = (
  <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" fill="currentColor" className="bi bi-file-earmark-arrow-up" viewBox="0 0 16 16">
    <path d="M8.5 11.5a.5.5 0 0 1-1 0V7.707L6.354 8.854a.5.5 0 1 1-.708-.708l2-2a.5.5 0 0 1 .708 0l2 2a.5.5 0 0 1-.708.708L8.5 7.707V11.5z"/>
    <path d="M14 14V4.5L9.5 0H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2zM9.5 3A1.5 1.5 0 0 0 11 4.5h2V14a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1h5.5v2z"/>
  </svg>
);

const LEVEL_SYMBOLS = {
  "ERROR": (<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" fill="currentColor" className="bi bi-exclamation-diamond" viewBox="0 0 16 16">
    <title>ERROR</title>
    <path d="M6.95.435c.58-.58 1.52-.58 2.1 0l6.515 6.516c.58.58.58 1.519 0 2.098L9.05 15.565c-.58.58-1.519.58-2.098 0L.435 9.05a1.482 1.482 0 0 1 0-2.098L6.95.435zm1.4.7a.495.495 0 0 0-.7 0L1.134 7.65a.495.495 0 0 0 0 .7l6.516 6.516a.495.495 0 0 0 .7 0l6.516-6.516a.495.495 0 0 0 0-.7L8.35 1.134z"/>
    <path d="M7.002 11a1 1 0 1 1 2 0 1 1 0 0 1-2 0zM7.1 4.995a.905.905 0 1 1 1.8 0l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 4.995z"/>
  </svg>),
  "WARN": (<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" fill="currentColor" className="bi bi-exclamation-triangle" viewBox="0 0 16 16">
    <title>WARN</title>
    <path d="M7.938 2.016A.13.13 0 0 1 8.002 2a.13.13 0 0 1 .063.016.146.146 0 0 1 .054.057l6.857 11.667c.036.06.035.124.002.183a.163.163 0 0 1-.054.06.116.116 0 0 1-.066.017H1.146a.115.115 0 0 1-.066-.017.163.163 0 0 1-.054-.06.176.176 0 0 1 .002-.183L7.884 2.073a.147.147 0 0 1 .054-.057zm1.044-.45a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566z"/>
    <path d="M7.002 12a1 1 0 1 1 2 0 1 1 0 0 1-2 0zM7.1 5.995a.905.905 0 1 1 1.8 0l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995z"/>
  </svg>),
  "INFO": (<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" fill="currentColor" className="bi bi-info-circle" viewBox="0 0 16 16">
    <title>INFO</title>
    <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"/>
    <path d="m8.93 6.588-2.29.287-.082.38.45.083c.294.07.352.176.288.469l-.738 3.468c-.194.897.105 1.319.808 1.319.545 0 1.178-.252 1.465-.598l.088-.416c-.2.176-.492.246-.686.246-.275 0-.375-.193-.304-.533L8.93 6.588zM9 4.5a1 1 0 1 1-2 0 1 1 0 0 1 2 0z"/>
  </svg>),
  "DEBUG": (<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" fill="currentColor" className="bi bi-bug" viewBox="0 0 16 16">
    <title>DEBUG</title>
    <path d="M4.355.522a.5.5 0 0 1 .623.333l.291.956A4.979 4.979 0 0 1 8 1c1.007 0 1.946.298 2.731.811l.29-.956a.5.5 0 1 1 .957.29l-.41 1.352A4.985 4.985 0 0 1 13 6h.5a.5.5 0 0 0 .5-.5V5a.5.5 0 0 1 1 0v.5A1.5 1.5 0 0 1 13.5 7H13v1h1.5a.5.5 0 0 1 0 1H13v1h.5a1.5 1.5 0 0 1 1.5 1.5v.5a.5.5 0 1 1-1 0v-.5a.5.5 0 0 0-.5-.5H13a5 5 0 0 1-10 0h-.5a.5.5 0 0 0-.5.5v.5a.5.5 0 1 1-1 0v-.5A1.5 1.5 0 0 1 2.5 10H3V9H1.5a.5.5 0 0 1 0-1H3V7h-.5A1.5 1.5 0 0 1 1 5.5V5a.5.5 0 0 1 1 0v.5a.5.5 0 0 0 .5.5H3c0-1.364.547-2.601 1.432-3.503l-.41-1.352a.5.5 0 0 1 .333-.623zM4 7v4a4 4 0 0 0 3.5 3.97V7H4zm4.5 0v7.97A4 4 0 0 0 12 11V7H8.5zM12 6a3.989 3.989 0 0 0-1.334-2.982A3.983 3.983 0 0 0 8 2a3.983 3.983 0 0 0-2.667 1.018A3.989 3.989 0 0 0 4 6h8z"/>
  </svg>),
  "TRACE": (<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" fill="currentColor" className="bi bi-broadcast" viewBox="0 0 16 16">
    <title>TRACE</title>
    <path d="M3.05 3.05a7 7 0 0 0 0 9.9.5.5 0 0 1-.707.707 8 8 0 0 1 0-11.314.5.5 0 0 1 .707.707zm2.122 2.122a4 4 0 0 0 0 5.656.5.5 0 1 1-.708.708 5 5 0 0 1 0-7.072.5.5 0 0 1 .708.708zm5.656-.708a.5.5 0 0 1 .708 0 5 5 0 0 1 0 7.072.5.5 0 1 1-.708-.708 4 4 0 0 0 0-5.656.5.5 0 0 1 0-.708zm2.122-2.12a.5.5 0 0 1 .707 0 8 8 0 0 1 0 11.313.5.5 0 0 1-.707-.707 7 7 0 0 0 0-9.9.5.5 0 0 1 0-.707zM10 8a2 2 0 1 1-4 0 2 2 0 0 1 4 0z"/>
  </svg>),
};

const range = (start, end) => {
  if (end === undefined) {
    end = start;
    start = 0;
  }

  return Array(end - start).fill(start).map((x, y) => x + y);
};

const upperBoundValue = (arr, val) => {
  if (arr.length === 0) {
    return null;
  } else if (arr.length === 1) {
    return arr[0] > val ? arr[0] : null;
  } else if (arr.length === 2) {
    return arr[0] > val ? arr[0] : arr[1] > val ? arr[1] : null;
  }

  var mid = Math.floor(arr.length / 2);

  if (arr[mid] <= val) {
    return upperBoundValue(arr.slice(mid, Number.MAX_VALUE), val);
  } else /*if (arr[mid] > val)*/ {
    return upperBoundValue(arr.slice(0, mid + 1), val);
  }
};

const randomChoice = (array) => (
  array[Math.floor(Math.random() * array.length)]
);

function asyncLogTimeOf(name, fun) {
  setTimeout(() => logTimeOf(name, fun), 0);
}

function logTimeOf(name, fun) {
  const t0 = performance.now();
  fun();
  const t1 = performance.now();
  if (DEBUG_PERF_LOG) {
    console.log(`Task "${name}" took ${t1 - t0} milliseconds.`);
  }
}

const keyToAddrCycle = (key) => {
  var [addr, cycle_no] = key.split(",");
  return { addr: addr, cycle_no: cycle_no };
};
