import React, {
  Component
} from 'react'
import withRouter from 'umi/withRouter'
import PropTypes from 'prop-types'
import {
  connect
} from 'dva'

import {
  get,
  failed
} from '../utils/request'
import {
  get as getToken
} from '../utils/token'
import Layout from './Layout'

class Widget extends Component {
  constructor(props) {
    super(props)
  }
  componentDidMount() {
    const {
      dispatch
    } = this.props

    // TODO
    var token = getToken()
    if (token) {
      dispatch({
        type: 'currentUser/sign-in',
        token
      })
    }
  }
  render() {
    const {
      children
    } = this.props
    return (<Layout>{children}</Layout>)
  }
}

Widget.propTypes = {
  children: PropTypes.node.isRequired,
}

export default withRouter(connect()(Widget))