import React, {Component} from 'react'
import {BrowserRouter as Router, Route, Switch} from 'react-router-dom'
import Loadable from 'react-loadable'
import {IntlProvider} from 'react-intl'
import {Provider} from 'react-redux'
import {createStore} from 'redux'
import moment from 'moment'
import {Spin, LocaleProvider} from 'antd'

import './App.css'
import pages from './pages'
import NoMatch from './pages/NoMatch'
import Layout from './layout'
import rootReducer from './reducers'
import {get as getLocale} from './intl'

const store = createStore(rootReducer)

const locale = getLocale()
moment.locale(locale.moment)

const Loading = () => <Spin size="large"/>

class Widget extends Component {
  render() {
    return (<Provider store={store}>
      <IntlProvider locale={locale.locale} messages={locale.messages}>
        <LocaleProvider locale={locale.antd}>
          <Router basename="/my">
            <Layout>
              <Switch>
                {
                  pages.map((it) => {
                    return (<Route key={it.path} path={it.path} exact={true} component={Loadable({loader: it.component, loading: Loading})}/>)
                  })
                }
                <Route component={NoMatch}/>
              </Switch>
            </Layout>
          </Router>
        </LocaleProvider>
      </IntlProvider>
    </Provider>);
  }
}

export default Widget
