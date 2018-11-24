import Header from './Header'
import Footer from './Footer'

export default function(props) {
  return (<div>
    <Header/> {props.children}
    <Footer/>
  </div>);
}
