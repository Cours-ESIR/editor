import styles from '../App.module.css';
import { createSignal, onMount } from 'solid-js';
import Editor from './Editor';
import Vr from './Vr';
import { invoke } from '@tauri-apps/api';

export default function () {
  let [x1, setx1] = createSignal<number>(200);
  let [x2, setx2] = createSignal<number>(200);

  let ref: HTMLDivElement | undefined;

  
  let [image, setImage] = createSignal<string>("");


  let render = ()=>{
    invoke("compile_project").then();

    let render: Promise<string> = invoke("render_project", { page: 0 })
    render.then((reponse) => {
      setImage(reponse)
    })
  }

  render()

  type Reponse = {
    message: string;
  }

  

  return (
    <div ref={ref!} class={styles.main}>
      <div style={"width :" + x1().toString() + "px;"}></div>
      <Vr max={() => { return 200 }} getX={x1} setX={setx1}></Vr>
      <div style={"width :" + x2().toString() + "px;"}>
        <Editor renderer={render}></Editor>
      </div>
      <Vr getX={x2} max={() => { return ref!.clientWidth - x1() - 24 }} setX={setx2}></Vr>
      <div style={"background: white; width : calc(100% - 24px - " + (x2() + x1()).toString() + "px);"} >
        <img src={`data:image/svg+xml;utf8,${encodeURIComponent(image())}`} />
      </div>
    </div>
  );
}

async function render(): Promise<string> {
  let render: Promise<string> = invoke("render_project", { page: 0 })
  return render
}
