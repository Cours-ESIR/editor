import type { Component } from 'solid-js';

import styles from './App.module.css';
import Toolbar from './components/Toolbar';
import Main from './components/Main';

const App: Component = () => {
  return (
    <div class={styles.app}>
      <Toolbar/>
      <Main/>
    </div>
  );
};





export default App;