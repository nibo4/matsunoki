import {Routes, Route, Router, useParams, Link, useLocation} from 'solid-app-router';
import { Suspense, Component, createResource, For, Switch, Match, JSXElement, Show, createMemo } from 'solid-js';
import { DefaultLayout } from './layout/default'
import styles from './App.module.css'
import zip from 'lodash.zip'

const storyName = (fileName: string): string => {
  const filePath = fileName
  const [_, ...path] = filePath.split('/');
  const baseName = path.slice(-1)[0].split('.')[0]
  return [...path.slice(0, -1), baseName].join(':')
}

const storyPath = (storyName: string): string => {
  return `./${storyName.split(':').join('/')}.tsx`
}

const Layout: Component<{children: JSXElement}> = (props) => {
  return (
    <div class={styles["layout"]}>
      <div class={styles["header"]}>
        <span>松の木プレビュー環境</span>
      </div>
      <div class={styles["container"]}>
        {props.children}
      </div>
    </div>
  )
}
const App: Component = () => {
  return (
    <Router>
      <Suspense fallback={<p>Loading stories...</p>}>
        <Catalog />
      </Suspense>
    </Router>
  );
};

const stories = (): string[]  => {
  return Object.keys(import.meta.glob("./**/**/*-story.tsx"))
}

const fetchStories = async (): Promise<Record<string, Component>> => {
  return Promise.all(stories().map(path => import(path).then(m => m.default))).then(components => {
    return Object.fromEntries(zip(stories(), components))
  })
}

const Catalog = () => {
  const [mods]  = createResource(fetchStories)
  const location = useLocation()
  const pathname = createMemo(() => location.pathname.split('').slice(1).join(''));


  return (
    <Switch fallback={<p>Loading</p>}>
      <Match when={mods()}>
        {mods => (
          <Layout>
            <div class={styles["side-bar"]}>
              <ul>
                <For each={stories()}>
                {
                  (item) => (
                    <li>
                      <Link href={storyName(item)}>{storyName(item)}</Link>
                      <Show when={storyName(item) == pathname()}><span>←</span></Show>
                    </li>
                  )
                }
                </For>
              </ul>
            </div>
            <div class={styles["contents"]}>
              <Routes>
                <Route path="/" element={<p>Hello world</p>}/>
                <Route path="/:name" element={<Content stories={mods} />}/>
              </Routes>
            </div>
          </Layout>
        )}
      </Match>
    </Switch>
  )
}

const Content: Component<{stories: Record<string, Component>}> = (props) => {
  const params = useParams()

  if(params.name) {
    return <>{props.stories[storyPath(params.name)]}</>
  }
  return <></>
}

export default App;
