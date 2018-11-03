import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {connect} from 'react-redux'
import {injectIntl, intlShape, FormattedMessage} from 'react-intl'
import HeaderSearch from 'ant-design-pro/lib/HeaderSearch'
import {
  Icon,
  Layout,
  Menu,
  message,
  Modal,
  Row
} from 'antd'
import {withRouter} from 'react-router-dom'
import {push} from 'connected-react-router'

import {userSignOut} from '../actions'
import {set as setLocale} from '../utils/locale'
import {get, delete_, failed} from '../utils/request'
import {
  is_sign_in,
  is_administrator,
  is_forum_manager,
  is_pos_manager,
  is_library_manager,
  is_hotel_manager
} from '../utils/authorized'
import {HOME} from '../utils/config'
import NoticeBar from './NoticeBar'
import Footer from './Footer'

const {Header, Sider, Content} = Layout

class Widget extends Component {
  constructor(props) {
    super(props)
    this.state = {
      languages: []
    }
  }
  handleHeaderClick = (e) => {
    const {push} = this.props
    const {formatMessage} = this.props.intl

    const lang = 'lang-';
    if (e.key.startsWith(lang)) {
      setLocale(e.key.substring(lang.length))
      return
    }

    switch (e.key) {
      case 'home':
        window.open("/", "_blank")
        return;
      case 'doc':
        window.open(HOME, "_blank")
        return
      case 'sign-in':
        push('/users/sign-in')
        return
      case 'toggle':
        this.setState({
          collapsed: !this.state.collapsed
        })
        return
      case 'sign-out':
        Modal.confirm({
          title: formatMessage({id: "header.sign-out.confirm"}),
          onOk() {
            delete_('/users/sign-out').then((rst) => {
              push('/users/sign-in')
              message.info(formatMessage({id: "flashes.success"}))
              userSignOut()
            }).catch(failed)
          }
        })
        return
      default:
        console.log(e.key)
    }
  }
  siderMenus = (auth) => {
    var items = []
    if (!is_sign_in(auth)) {
      return items
    }

    items.push({
      icon: 'user',
      label: 'nut.dashboard.self.title',
      children: [
        {
          label: 'nut.users.logs.title',
          to: '/users/logs'
        }, {
          label: 'nut.users.profile.title',
          to: '/users/profile'
        }, {
          label: 'nut.users.change-password.title',
          to: '/users/change-password'
        }
      ]
    })

    if (is_administrator(auth)) {
      items.push({
        icon: 'setting',
        label: 'nut.dashboard.site.title',
        children: [
          {
            label: 'nut.admin.site.status.title',
            to: '/admin/site/status'
          }, {
            label: 'nut.admin.site.info.title',
            to: '/admin/site/info'
          }, {
            label: 'nut.admin.site.author.title',
            to: '/admin/site/author'
          }, {
            label: 'nut.admin.site.seo.title',
            to: '/admin/site/seo'
          }, {
            label: 'nut.admin.site.smtp.title',
            to: '/admin/site/smtp'
          }, {
            label: 'nut.admin.users.index.title',
            to: '/admin/users'
          }, {
            label: 'nut.admin.members.index.title',
            to: '/admin/members'
          }, {
            label: 'nut.admin.locales.index.title',
            to: '/admin/locales'
          }, {
            label: 'nut.admin.friend-links.index.title',
            to: '/admin/friend-links'
          }, {
            label: 'nut.admin.links.index.title',
            to: '/admin/links'
          }, {
            label: 'nut.admin.cards.index.title',
            to: '/admin/cards'
          }, {
            label: 'nut.admin.leave-words.index.title',
            to: '/admin/leave-words'
          }
        ]
      })
    }
    var forum = {
      icon: 'share-alt',
      label: 'forum.dashboard.title',
      children: [
        {
          label: 'forum.topics.index.title',
          to: '/forum/topics'
        }, {
          label: 'forum.posts.index.title',
          to: '/forum/posts'
        }
      ]
    }
    if (is_forum_manager(auth)) {
      forum.children.push({label: 'forum.tags.index.title', to: '/forum/tags'})
    }
    items.push(forum)
    items.push({icon: 'book', label: 'cbeta.dashboard.title', children: []})
    if (is_library_manager(auth)) {
      items.push({icon: 'idcard', label: 'library.dashboard.title', children: []})
    }
    if (is_hotel_manager(auth)) {
      items.push({icon: 'fork', label: 'hotel.dashboard.title', children: []})
    }

    items.push({icon: 'shopping-cart', label: 'shop.dashboard.title', children: []})

    if (is_pos_manager(auth)) {
      items.push({icon: 'qrcode', label: 'pos.dashboard.title', children: []})
    }

    items.push({icon: 'paper-clip', label: 'todo.dashboard.title', children: []})

    items.push({
      icon: 'team',
      label: 'caring.dashboard.title',
      children: [
        {
          label: 'caring.topics.index.title',
          to: '/caring/topics'
        }, {
          label: 'caring.posts.index.title',
          to: '/caring/posts'
        }
      ]
    })

    if (is_administrator(auth)) {
      items.push({icon: 'bank', label: 'donate.dashboard.title', children: []})
      items.push({icon: 'usb', label: 'ops.vpn.dashboard.title', children: []})
      items.push({icon: 'mail', label: 'ops.email.dashboard.title', children: []})
    }
    return items
  }
  headerMenus = (auth) => {
    const {formatMessage} = this.props.intl
    var items = []
    if (is_sign_in(auth)) {
      items.push({key: "sign-out", children: (<Icon type="logout"/>)})
      items.push({key: "notice-bar", children: (<NoticeBar/>)})
      items.push({key: "search", children: (<HeaderSearch placeholder={formatMessage({id: "header.search.placeholder"})}/>)})
    } else {
      items.push({key: "sign-in", children: (<Icon type="login"/>)})
    }
    items.push({key: "doc", children: (<Icon type="question-circle-o"/>)})
    return items
  }
  componentDidMount() {
    get('/site/info').then((data) => {
      this.setState({languages: data.languages})
    }).catch(failed)
  }
  render() {
    const {children, currentUser, push} = this.props

    return (<Layout>
      <Sider breakpoint="lg" collapsedWidth="0" trigger={null} collapsible="collapsible" collapsed={this.state.collapsed}>
        <div className="sider-logo"/>
        <Menu onClick={(e) => push(e.key)} theme="dark" mode="inline" defaultSelectedKeys={['1']}>
          {
            this.siderMenus(currentUser).map((it) => (<Menu.SubMenu key={it.label} title={(<span><Icon type={it.icon}/><FormattedMessage id={it.label}/></span>)}>
              {
                it.children.map((jt) => (<Menu.Item key={jt.to}>
                  <FormattedMessage id={jt.label}/>
                </Menu.Item>))
              }
            </Menu.SubMenu>))
          }
        </Menu>
      </Sider>
      <Layout>
        <Header style={{
            background: '#fff',
            padding: 0
          }}>
          <Menu onClick={this.handleHeaderClick} mode="horizontal">
            <Menu.Item key='toggle'>
              <Icon className="trigger" type={this.state.collapsed
                  ? 'menu-unfold'
                  : 'menu-fold'}/>
            </Menu.Item>
            {
              this.headerMenus(currentUser).map((it) => (<Menu.Item style={{
                  float: 'right'
                }} key={it.key}>
                {it.children}
              </Menu.Item>))
            }
            <Menu.SubMenu style={{
                float: 'right'
              }} key="switch-languages" title={<Icon type = "global" />}>
              {
                this.state.languages.map((it) =>< Menu.Item key = {
                  `lang-${it}`
                } > <FormattedMessage id={`languages.${it}`}/> < /Menu.Item>)}
            </Menu.SubMenu>
          </Menu>
        </Header>
        <Content style={{
            margin: '24px 16px',
            padding: 24,
            background: '#fff',
            minHeight: 360
          }}>
          <Row>{children}</Row>
        </Content>
        <Footer/>
      </Layout>
    </Layout>)
  }
}

Widget.propTypes = {
  children: PropTypes.node.isRequired,
  intl: intlShape.isRequired,
  currentUser: PropTypes.object.isRequired,
  push: PropTypes.func.isRequired,
  userSignOut: PropTypes.func.isRequired
}

export default withRouter(connect((state) => ({currentUser: state.currentUser}), {push, userSignOut})(injectIntl(Widget)))
