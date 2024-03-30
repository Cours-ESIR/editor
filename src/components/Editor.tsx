import styles from '~/styles/App.module.css';
import type { Component } from 'solid-js';

import { createCodeMirror } from "solid-codemirror";
import { createSignal, onMount } from "solid-js";
import * as monaco from "monaco-editor"

const Editor: Component = () => {
    let div:HTMLDivElement = document.createElement("div")
    let editor = monaco.editor.create(div,{
        theme: "vs-dark",
        minimap:{
            enabled:false
        },
        automaticLayout:true,
        // value:textvalue
    })
    return (
        div
    )
}
export default Editor
