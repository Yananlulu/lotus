import React, {Component} from 'react'
import {FormattedMessage} from 'react-intl'
import {DatePicker} from 'antd'

class Widget extends Component {
  render() {
    return (<div>
      <FormattedMessage id="install.title"/>
      <br/>
      <DatePicker/>
    </div>)
  }
}

export default Widget
