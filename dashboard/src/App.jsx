import React, {Component} from 'react'
import {BrowserRouter as Router, Route, Switch} from "react-router-dom"
import Loadable from "react-loadable"
import {Spin} from 'antd'

import './App.css'
import pages from './pages'
import NoMatch from './pages/NoMatch'

const Loading = () => <Spin size="large"/>

class Widget extends Component {
  render() {
    return (<Router basename="/my">
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
    </Router>);
  }
}

export default Widget
