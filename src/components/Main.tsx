import styles from '../App.module.css';
import { createSignal, type Component } from 'solid-js';
import Editor from './Editor';
import Vr from './Vr';

const Main: Component = () => {
    let [x1, setx1] = createSignal<number>(200)
    let [x2, setx2] = createSignal<number>(200)

    let ref:HTMLDivElement

    return (
      <div ref={ref} class={styles.main}>

        <div style={"width :" + x1().toString() + "px;"}></div>

        <Vr max={() => {return 200}} getX={x1} setX={setx1}></Vr>

        <div style={"width :" + x2().toString() + "px;"}>
          <Editor></Editor>
        </div>
        
        <Vr getX={x2} max={() => {return ref.clientWidth - x1() - 24 }} setX={setx2}></Vr>

        <div style={"width : calc(100% - 24px - " + (x2()+x1()).toString() + "px);"} >
        </div>
      </div>
    );
  };

export default Main