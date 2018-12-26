import React, {Component} from 'react'
import PropTypes from 'prop-types'
import moment from 'moment'

class Widget extends Component {
  render() {
    const {value} = this.props
    return (<span>{moment(value).fromNow()}</span>)
  }
}

Widget.propTypes = {
  name: PropTypes.string.isRequired
}

export default Widget
