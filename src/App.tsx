import type { Component } from 'solid-js';

import styles from './App.module.css';
import Sidebar from './components/Sidebar';
import Toolbar from './components/Toolbar';
import Main from './components/Main';

const App: Component = () => {
  return (
    <div class={styles.app}>
      <Sidebar/>
      <Toolbar/>
      <Main/>
    </div>
  );
};





export default App;