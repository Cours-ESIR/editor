import styles from '~/styles/App.module.css';
import type { Component } from 'solid-js';

import { createSignal, onMount } from "solid-js";
import * as monaco from "monaco-editor"

import { fs } from "@tauri-apps/api";
import { invoke } from "@tauri-apps/api";


interface inProps{
    path: string,
    renderer: Function
}


async function initEditor(div: HTMLDivElement, path: string | undefined, renderer: Function) {
    let [texte, setTexte] = createSignal("= caca \n $1/2$")

    if (path) {
        let buff = (await fs.readBinaryFile(path))
        let buff2 = buff as unknown as number[]
        setTexte(String.fromCharCode.apply(null, buff2))
    }

    let editor = monaco.editor.create(div, {
        theme: "vs-dark",
        minimap: {
            enabled: false
        },
        automaticLayout: true,
        value: texte()
    })


    div.oninput = () => {
        setTexte(editor.getValue())
        invoke("update_project", { content: texte() }).then()
        renderer()
    }

}


export default (props:inProps) => {
    const [path] = createSignal();

    let div: HTMLDivElement = document.createElement("div")
    div.style.height = "100%"
    initEditor(div, props.path, props.renderer)

    return (
        div
    )
}