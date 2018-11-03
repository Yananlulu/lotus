import React from 'react'
import ReactDOM from 'react-dom'
import {Provider} from 'react-redux'
import {applyMiddleware, compose, createStore} from 'redux'
import {addLocaleData, IntlProvider} from 'react-intl'
import {LocaleProvider} from 'antd'
import {createBrowserHistory} from 'history'
import {connectRouter, routerMiddleware} from 'connected-react-router'

import './main.css'
import App from './App'
import reducers from './reducers'
import {get, failed} from './utils/request'
import {detect as detectLocale} from './utils/locale'

const main = (node) => {
  const history = createBrowserHistory({basename: '/my/'})
  const store = createStore(connectRouter(history)(reducers), {}, compose(applyMiddleware(routerMiddleware(history))))
  const intl = detectLocale()

  get(`/locales/${intl.locale}`).then((data) => {
    addLocaleData(intl.data)

    ReactDOM.render((<Provider store={store}>
      <IntlProvider locale={intl.locale} messages={data.reduce((ar, it) => {
          ar[it.code] = it.message
          return ar
        }, {})}>
        <LocaleProvider locale={intl.antd}>
          <App history={history}/>
        </LocaleProvider>
      </IntlProvider>
    </Provider>), node)
  }).catch(failed)
}

export default main
