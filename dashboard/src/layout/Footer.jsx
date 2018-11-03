import React, {Component} from 'react'
import GlobalFooter from 'ant-design-pro/lib/GlobalFooter'
import {FormattedMessage} from 'react-intl'
import {Icon} from 'antd'

import {HOME} from '../utils/config'

const Copyright = () => (<div>
  <Icon type="copyright"/>
  <FormattedMessage id="site.copyright"/>
</div>)

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      items: [
        {
          key: 'help',
          title: <FormattedMessage id="footer.help"/>,
          href: `${HOME}/blob/master/README.md`,
          blankTarget: true
        }, {
          key: 'github',
          title: <Icon type="github"/>,
          href: HOME,
          blankTarget: true
        }, {
          key: 'license',
          title: <FormattedMessage id="footer.license"/>,
          href: `${HOME}/blob/master/LICENSE`,
          blankTarget: true
        }
      ]
    }
  }
  render() {
    return (<GlobalFooter links={this.state.items} copyright={<Copyright/>}/>)
  }
}

export default Widget
