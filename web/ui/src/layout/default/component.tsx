import { Component, JSXElement } from 'solid-js'
import Logo from '../../assets/logo.svg'
import styles from './styles.module.css'

const Navbar: Component = () => {
  return (
    <nav class={styles["navbar"]}>
      <Logo height="60px" width="200px"/>
    </nav>
  )
}
export const DefaultLayout: Component<{children: JSXElement}> = (props) => {
  return (
    <div class={styles["layout"]}>
      <header><Navbar /></header>
      <main class={styles["contents"]}>
        {props.children}
      </main>
    </div>
  )
}
