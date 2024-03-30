import styles from '~/styles/App.module.css';
import type { Component } from 'solid-js';

const Preview: Component = () => {
    return (
        <div class={styles.preview}>
            <div class={styles.scrolable}>
            </div>
        </div>
    );
};

export default Preview
