import type { Accessor, Setter } from 'solid-js';

interface IVRProps {
    max?: () => number;
    min?: () => number;

    getX: Accessor<number>;
    setX: Setter<number>;
}

export default function(props: IVRProps) {
    function move(e: PointerEvent) {
        props.setX( Math.min( Math.max( min() , props.getX() - ( px - e.screenX ) ), max()) );
        px = e.screenX;
    }

    function start(e: PointerEvent) {
        window.onpointermove = move;
        window.onpointerup = stop;
        px = e.screenX;
    }

    function stop() {
        window.onpointerup = () => {};
        window.onpointermove = () => {};
    }

    let px = 0;
    
    let min = props.min ? props.min : () => { return 0; };
    let max = props.max ? props.max : () => { return Infinity; }

    return (
        <div draggable="false" style="cursor:ew-resize;height:100%;background-color:rgba(255,255,255,0.1); width:12px;min-width:12px;position:relative;" onpointerdown={start}>
            <i draggable="false" style="top:50%;left:50%;position:absolute;transform:translate(-50%);font-size:18px; color:rgba(255,255,255,0.5);" class="ph ph-dots-six-vertical"></i>
        </div>
    );
}
