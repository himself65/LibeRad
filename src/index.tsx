import {} from 'react-dom/experimental'
import React from 'react'
import ReactDOM from 'react-dom'
import App from './App'
import * as serviceWorker from './serviceWorker'

if (DEBUG) {
  console.log('---DEVELOPMENT ENV---')
} else {
  console.log('---PRODUCTION ENV---')
}

const root = document.getElementById('root')!
ReactDOM.createRoot(root).render(<App/>)

serviceWorker.register()
