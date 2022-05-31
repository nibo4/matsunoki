import {Routes, Route, Router, useParams, Link} from 'solid-app-router';
import { Suspense, Component, createResource, For, Switch, Match } from 'solid-js';
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

const App: Component = () => {
  return (
    <Suspense fallback={<p>Loading stories...</p>}>
      <Catalog />
    </Suspense>
  );
};

const stories = (): string[]  => {
  return Object.keys(import.meta.glob("./page/**/*-story.tsx"))
}

const fetchStories = async (): Promise<Record<string, Component>> => {
  return Promise.all(stories().map(path => import(path).then(m => m.default))).then(components => {
    return Object.fromEntries(zip(stories(), components))
  })
}

const Catalog = () => {
  const [mods]  = createResource(fetchStories)

  return (
    <Switch fallback={<p>Loading</p>}>
      <Match when={mods()}>
        <Router>
          <div class={styles["container"]}>
            <div class={styles["side-bar"]}>
              <ul>
                <For each={stories()}>
                {
                  (item) => <li><Link href={storyName(item)}>{storyName(item)}</Link></li>
                }
                </For>
              </ul>
            </div>
            <div class={styles["contents"]}>
              <Routes>
                <Route path="/" element={<p>Hello world</p>}/>
                <Route path="/:name" element={<Content stories={mods() as any} />}/>
              </Routes>
            </div>
          </div>
        </Router>
      </Match>
    </Switch>
  )
}

const Content: Component<{stories: Record<string, Component>}> = (props) => {
  const params = useParams()
  console.dir(params)
  if(params.name) {
    return <>{props.stories[storyPath(params.name)]}</>
  }
  return <></>
}

export default App;
