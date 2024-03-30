import styles from '~/styles/App.module.css';
import type { Component } from 'solid-js';
import Editor from "./Main/Editor"
import Preview from "./Main/Preview"
import AppState from '../lib/AppState';

const Main: Component = (app_state: AppState) => {
    return (
        <div class={styles.main}>
            <Editor />
            <div class={styles.spliter}></div>
            <Preview />
        </div>
    );
};

export default Main