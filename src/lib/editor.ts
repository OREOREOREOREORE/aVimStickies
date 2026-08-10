import { Compartment, EditorState, type Extension } from "@codemirror/state";
import {
  EditorView,
  keymap,
  lineNumbers,
  highlightActiveLineGutter,
  highlightSpecialChars,
  drawSelection,
  dropCursor,
  rectangularSelection,
  crosshairCursor,
  highlightActiveLine,
} from "@codemirror/view";
import { history, defaultKeymap, historyKeymap } from "@codemirror/commands";
import {
  syntaxHighlighting,
  defaultHighlightStyle,
  indentOnInput,
  bracketMatching,
  foldGutter,
  foldKeymap,
} from "@codemirror/language";
import {
  closeBrackets,
  closeBracketsKeymap,
  autocompletion,
  completionKeymap,
} from "@codemirror/autocomplete";
import { searchKeymap, highlightSelectionMatches } from "@codemirror/search";
import { markdown } from "@codemirror/lang-markdown";
import { vim } from "@replit/codemirror-vim";

const baseExtensions = [
  highlightSpecialChars(),
  history(),
  drawSelection(),
  dropCursor(),
  EditorState.allowMultipleSelections.of(true),
  indentOnInput(),
  syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
  bracketMatching(),
  closeBrackets(),
  autocompletion(),
  rectangularSelection(),
  crosshairCursor(),
  highlightActiveLine(),
  highlightSelectionMatches(),
  keymap.of([
    ...closeBracketsKeymap,
    ...defaultKeymap,
    ...searchKeymap,
    ...historyKeymap,
    ...foldKeymap,
    ...completionKeymap,
  ]),
];

export interface NoteEditor {
  getContent(): string;
  setDoc(doc: string): void;
  setReadOnly(ro: boolean): void;
  applyLineNumbers(show: boolean): void;
  applyWrap(show: boolean): void;
  applyTheme(dark: boolean): void;
  focus(): void;
  requestMeasure(): void;
  destroy(): void;
}

export function createNoteEditor(
  parent: HTMLElement,
  opts: {
    doc: string;
    showLineNumbers: boolean;
    wrapText: boolean;
    dark: boolean;
    onChange?: (doc: string) => void;
  }
): NoteEditor {
  const readOnlyComp = new Compartment();
  const lineNumbersComp = new Compartment();
  const wrapComp = new Compartment();
  const themeComp = new Compartment();

  const view = new EditorView({
    state: EditorState.create({
      doc: opts.doc,
      extensions: [
        ...baseExtensions,
        readOnlyComp.of(EditorState.readOnly.of(false)),
        lineNumbersComp.of(lineNumberExts(opts.showLineNumbers)),
        wrapComp.of(opts.wrapText ? EditorView.lineWrapping : []),
        themeComp.of(EditorView.theme({}, { dark: opts.dark })),
        markdown(),
        vim(),
        EditorView.updateListener.of((u) => {
          if (u.docChanged && !silent) opts.onChange?.(u.state.doc.toString());
        }),
      ],
    }),
    parent,
  });

  let silent = false;

  return {
    getContent: () => view.state.doc.toString(),
    setDoc: (doc: string) => {
      const current = view.state.doc.toString();
      if (doc !== current) {
        silent = true;
        view.dispatch({ changes: { from: 0, to: current.length, insert: doc } });
        silent = false;
      }
    },
    setReadOnly: (ro: boolean) => {
      view.dispatch({ effects: readOnlyComp.reconfigure(EditorState.readOnly.of(ro)) });
    },
    applyLineNumbers: (show: boolean) => {
      view.dispatch({ effects: lineNumbersComp.reconfigure(lineNumberExts(show)) });
    },
    applyWrap: (show: boolean) => {
      view.dispatch({
        effects: wrapComp.reconfigure(show ? EditorView.lineWrapping : []),
      });
    },
    applyTheme: (dark: boolean) => {
      view.dispatch({
        effects: themeComp.reconfigure(EditorView.theme({}, { dark })),
      });
    },
    focus: () => view.focus(),
    requestMeasure: () => view.requestMeasure(),
    destroy: () => view.destroy(),
  };
}

function lineNumberExts(show: boolean): Extension[] {
  return show ? [lineNumbers(), highlightActiveLineGutter(), foldGutter()] : [];
}
