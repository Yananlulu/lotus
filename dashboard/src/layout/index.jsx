import React, {Component} from 'react'
import PropTypes from 'prop-types'
import {connect} from 'react-redux'
import {withRouter} from 'react-router-dom'

import Header from './Header'
import Footer from './Footer'
import {signIn} from '../actions'

class Widget extends Component {
  render() {
    const {children} = this.props
    return (<div>
      <Header/> {children}
      <Footer/>
    </div>)
  }
}

Widget.propTypes = {
  children: PropTypes.node.isRequired,
  user: PropTypes.object.isRequired,
  signIn: PropTypes.func.isRequired
}

const mapStateToProps = state => ({user: state.currentUser})

const mapDispatchToProps = dispatch => ({
  signIn: token => dispatch(signIn(token))
})

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(Widget))
