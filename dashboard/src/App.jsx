import React, {Component} from 'react'
import {BrowserRouter as Router, Route, Switch} from 'react-router-dom'
import Loadable from 'react-loadable'
import {IntlProvider} from 'react-intl'
import {Spin, LocaleProvider} from 'antd'
import moment from 'moment'

import './App.css'
import pages from './pages'
import NoMatch from './pages/NoMatch'
import {get as getLocale} from './intl'

const Loading = () => <Spin size="large"/>

const locale = getLocale()
moment.locale(locale.moment)

class Widget extends Component {
  render() {
    return (<IntlProvider locale={locale.locale} messages={locale.messages}>
      <LocaleProvider locale={locale.antd}>
        <Router basename="/my">
          <div>
            <Switch>
              {
                pages.map((it) => {
                  return (<Route key={it.path} path={it.path} exact={true} component={Loadable({loader: it.component, loading: Loading})}/>)
                })
              }
              <Route component={NoMatch}/>
            </Switch>
          </div>
        </Router>
      </LocaleProvider>
    </IntlProvider>);
  }
}

export default Widget
