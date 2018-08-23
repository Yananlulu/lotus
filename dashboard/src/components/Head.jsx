import React, {
  Component
} from 'react'
import PropTypes from 'prop-types'
import {
  Helmet
} from "react-helmet"
import {
  formatMessage
} from 'umi/locale'

class Widget extends Component {
  render() {
    return (<Helmet>
      <title>
        {formatMessage(this.props.title)}|{formatMessage({id: 'site.subhead'})}|{formatMessage({id: 'site.title'})}
      </title>
    </Helmet>);
  }
}

Widget.propTypes = {
  title: PropTypes.object.isRequired
}

export default Widget