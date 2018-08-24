import React, {Component} from 'react'
import {BrowserRouter as Router, Route, Switch} from "react-router-dom"

import plugins from './plugins'
import dynamicWrapper from './utils/loadable'

class Widget extends Component {
  render() {
    return (<Router basename="/my">
      <Switch>
        {plugins.routes.map((it) => (<Route key={it.path} path={it.path} exact={true} component={dynamicWrapper(it.component)}/>))}
        <Route component={dynamicWrapper(() => import ('./NotFound'))}/>
      </Switch>
    </Router>)
  }
}

export default Widget
