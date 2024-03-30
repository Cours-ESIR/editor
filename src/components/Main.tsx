import styles from '../App.module.css';
import type { Component } from 'solid-js';
import Editor from './Editor';

const Main: Component = () => {
    return (
      <div class={styles.main}>
        <Editor></Editor>
        <div class={styles.preview}>
          <div class={styles.scrolable}>
          </div>
        </div>
      </div>
    );
  };

export default Main