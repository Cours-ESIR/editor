import { createSignal, type Component } from 'solid-js';

import styles from '~/styles/App.module.css';
import SideBar from './components/Sidebar';
import ToolBar from './components/Toolbar';
import ModeLine from './components/ModeLine';
import Main from './components/Main';
import AppState from './lib/AppState';

const App: Component = () => {
  const [app_state, set_app_state] = createSignal(AppState);

  return (
    <div class={`${styles.app} ${styles.row_flex}`}>
      <SideBar />
      <div class={`${styles.col_flex} ${styles.flex_grow}`}>
        <ToolBar />
        <Main app_state={app_state} />
        <ModeLine />
      </div>
    </div>
  );
};

export default App;