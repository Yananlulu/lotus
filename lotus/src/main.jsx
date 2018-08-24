import React from 'react'
import ReactDOM from 'react-dom'
import {Provider} from 'react-redux'
import {createStore} from 'redux'

import './main.css'
import App from './App'
import reducers from './reducers'

const store = createStore(reducers)

const main = (node) => {
  ReactDOM.render((<Provider store={store}>
    <App/>
  </Provider>), node)
}

export default main
