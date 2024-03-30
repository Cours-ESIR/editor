import styles from '~/styles/App.module.css';
import type { Component } from 'solid-js';

import { createCodeMirror } from "solid-codemirror";
import { createSignal, onMount } from "solid-js";
import { EditorView, lineNumbers } from '@codemirror/view';
import { basicSetup } from 'codemirror';
import { Transaction } from '@codemirror/state';
import { javascript, javascriptLanguage, tsxLanguage } from '@codemirror/lang-javascript';
import { CompletionContext } from "@codemirror/autocomplete"
import { history } from "@codemirror/commands"
import { search } from "@codemirror/search"
import { syntaxHighlighting, codeFolding } from "@codemirror/language"
import { oneDarkHighlightStyle } from "@codemirror/theme-one-dark"

const Editor: Component = () => {
    const { editorView, ref: editorRef, createExtension } = createCodeMirror({
        /**
         * The initial value of the editor
         */
        value: "console.log('hello world!')",
        /**
         * Fired whenever the editor code value changes.
         */
        onValueChange: (value) => console.log("value changed", value),
        /**
         * Fired whenever a change occurs to the document, every time the view updates.
         */
        onModelViewUpdate: (modelView) => console.log("modelView updated", modelView),
        /**
         * Fired whenever a transaction has been dispatched to the view.
         * Used to add external behavior to the transaction [dispatch function](https://codemirror.net/6/docs/ref/#view.EditorView.dispatch) for this editor view, which is the way updates get routed to the view
         */
        onTransactionDispatched: (tr: Transaction, view: EditorView) => console.log("Transaction", tr)
    });
    const theme = EditorView.theme({
        ".cm-content": {
            caretColor: "#528bff",
        },

        "&": {
            color: "#abb2bf",
            backgroundColor: "#21252b",
        },

        ".cm-gutters": {
            backgroundColor: "#21252b",
            color: "#7d8799",
            border: "none"
        },

        ".cm-activeLineGutter": {
            backgroundColor: "#A12510",
            color: "#BdB7B9",
            border: "none"
        },
    });
    
    function myCompletions(context: CompletionContext) {
        let word = context.matchBefore(/\w*/)
        if (word == null)
            return null
        if (word.from == word.to && !context.explicit)
            return null
        return {
            from: word.from,
            options: [
                { label: "match", type: "keyword" },
                { label: "hello", type: "variable", info: "(World)" },
                { label: "magic", type: "text", apply: "⠁⭒*.✩.*⭒⠁", detail: "macro" }
            ]
        }
    }

    createExtension(theme);
    createExtension(syntaxHighlighting(oneDarkHighlightStyle));
    createExtension(basicSetup);
    createExtension(lineNumbers);
    // createExtension(history);
    // createExtension(search);
    // createExtension(codeFolding);
    createExtension(javascript);
    return <div ref={editorRef} style={"height: 100%;"} />;

    // return (
    //     // <div class={styles.editor}>
    //     //     <div class={styles.scrolable}>
    //     //         <div class={`${styles.content}`} contentEditable={true} role='textbox'>
    //     //         </div>
    //     //     </div>
    //     // </div>



    // );
};

export default Editor
