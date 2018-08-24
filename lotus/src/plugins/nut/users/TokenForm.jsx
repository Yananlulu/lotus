import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {injectIntl, intlShape} from 'react-intl'
import {message, Spin} from 'antd'
import {withRouter} from 'react-router-dom'

import {client, failed} from '../../utils/request'

class Widget extends Component {
  componentDidMount() {
    const {action, query, match} = this.props
    const {formatMessage} = this.props.intl

    client().request(query, match.params).then((rst) => {
      message.info(formatMessage({id: `nut.emails.user.${action}.success`}))
      router.push('/users/sign-in')
    }).catch(failed)
  }
  render() {
    return (<Spin size="large"/>)
  }
}

Widget.propTypes = {
  intl: intlShape.isRequired,
  action: PropTypes.string.isRequired,
  query: PropTypes.string.isRequired
}

export default withRouter(injectIntl(Widget))