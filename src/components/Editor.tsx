import styles from '~/styles/App.module.css';
import type { Component } from 'solid-js';

import { createSignal, onMount } from "solid-js";
import * as monaco from "monaco-editor"


import { fs } from "@tauri-apps/api";
import { invoke } from "@tauri-apps/api";


async function initEditor(div: HTMLDivElement, path: string | undefined) {
    let textvalue = "= caca \n $1/2$"

    if (path) {
        let buff = (await fs.readBinaryFile(path))
        let buff2 = buff as unknown as number[]
        textvalue = String.fromCharCode.apply(null, buff2)
    }

    let editor = monaco.editor.create(div, {
        theme: "vs-dark",
        minimap: {
            enabled: false
        },
        automaticLayout: true,
        value: textvalue
    })
    invoke("update_project", { content: textvalue }).then();

}


const Editor: Component = (props) => {
    const [path] = createSignal();

    let div: HTMLDivElement = document.createElement("div")
    div.style.height = "100%"
    initEditor(div, props.path)

    return (
        div
    )
}

export default Editor