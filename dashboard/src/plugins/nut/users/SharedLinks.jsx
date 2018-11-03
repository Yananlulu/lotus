import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {Icon, Menu} from 'antd'
import {FormattedMessage} from 'react-intl'
import {connect} from 'react-redux'
import {push} from 'connected-react-router'

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      items: [
        {
          icon: 'login',
          label: 'nut.users.sign-in.title',
          to: '/users/sign-in'
        }, {
          icon: 'user-add',
          label: 'nut.users.sign-up.title',
          to: '/users/sign-up'
        }, {
          icon: 'retweet',
          label: 'nut.users.forgot-password.title',
          to: '/users/forgot-password'
        }, {
          icon: 'check',
          label: 'nut.users.confirm.title',
          to: '/users/confirm'
        }, {
          icon: 'unlock',
          label: 'nut.users.unlock.title',
          to: '/users/unlock'
        }, {
          icon: 'message',
          label: 'nut.leave-words.new.title',
          to: '/leave-words/new'
        }
      ]
    }
  }
  render() {
    const {push} = this.props
    return (<Menu onClick={(e) => push(e.key)} mode="inline">
      {
        this.state.items.map((it) => (<Menu.Item key={it.to}>
          <Icon type={it.icon}/>
          <FormattedMessage id={it.label}/>
        </Menu.Item>))
      }
    </Menu>)
  }
}

Widget.propTypes = {
  push: PropTypes.func.isRequired
}

export default connect((state) => ({}), {push})(Widget)
