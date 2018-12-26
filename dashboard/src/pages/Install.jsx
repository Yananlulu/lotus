import React, {Component} from 'react'
import {FormattedMessage} from 'react-intl'
import {DatePicker} from 'antd'

import SharedLinks from './users/SharedLinks'

class Widget extends Component {
  render() {
    return (<div>
      <FormattedMessage id="install.title"/>
      <br/>
      <DatePicker/>
      <br/>
      <SharedLinks/>
    </div>)
  }
}

export default Widget
