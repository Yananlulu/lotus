import React from 'react'
import ReactDOM from 'react-dom'
import {Provider} from 'react-redux'
import {createStore} from 'redux'
import {addLocaleData, IntlProvider} from 'react-intl'
import {LocaleProvider} from 'antd'

import './main.css'
import App from './App'
import reducers from './reducers'
import {get, failed} from './utils/request'
import {detect as detectLocale} from './utils/locale'

const store = createStore(reducers)

const main = (node) => {
  const intl = detectLocale()

  get(`/locales/${intl.locale}`).then((data) => {
    addLocaleData(intl.data)

    ReactDOM.render((<Provider store={store}>
      <IntlProvider locale={intl.locale} messages={data.reduce((ar, it) => {
          ar[it.code] = it.message
          return ar
        }, {})}>
        <LocaleProvider locale={intl.antd}>
          <App/>
        </LocaleProvider>
      </IntlProvider>
    </Provider>), node)
  }).catch(failed)
}

export default main
