import styles from '../App.module.css';
import type { Component } from 'solid-js';

const Main: Component = () => {
    return (
      <div class={styles.main}>
        <div class={styles.editor}>
          <div class={styles.scrolable}>
            <div class={styles.content} contentEditable={true} role='textbox'>
              Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc nec orci eleifend, feugiat enim sed, varius massa. Vivamus erat lectus, semper ut porttitor id, ultricies eget sem. Ut malesuada nunc eu iaculis mattis. Duis sed metus at magna ultricies scelerisque sollicitudin sit amet augue. Mauris porta at velit et pellentesque. Donec pretium id urna a dignissim. Sed ac sapien non mauris aliquet tincidunt. Fusce eu viverra risus. In hac habitasse platea dictumst. Donec vehicula bibendum sapien non sollicitudin. Proin in laoreet metus. Nulla pulvinar tincidunt lorem vitae blandit. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Duis ac aliquet urna. Quisque feugiat, nulla ut convallis congue, neque ligula dictum quam, non tincidunt dolor leo vel orci.
            </div>
          </div>
        </div>
        <div class={styles.preview}>
          <div class={styles.scrolable}>
          </div>
        </div>
      </div>
    );
  };

export default Main