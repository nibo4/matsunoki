import { Component, JSXElement } from 'solid-js'
import styles from './styles.module.css'
import Logo from '../../assets/logo.svg'

const Navbar = () => {
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
