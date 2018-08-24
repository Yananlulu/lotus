import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {Route, Switch} from "react-router-dom"
import {ConnectedRouter} from 'connected-react-router'

import plugins from './plugins'
import dynamicWrapper from './utils/loadable'
import Layout from './layout'

class Widget extends Component {
  render() {
    const {history} = this.props

    return (<ConnectedRouter history={history}>
      <Layout>
        <Switch>
          {plugins.routes.map((it) => (<Route key={it.path} path={it.path} exact={true} component={dynamicWrapper(it.component)}/>))}
          <Route component={dynamicWrapper(() => import ('./NotFound'))}/>
        </Switch>
      </Layout>
    </ConnectedRouter>)
  }
}

Widget.propTypes = {
  history: PropTypes.object.isRequired
}

export default Widget
