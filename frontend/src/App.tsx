import React from 'react';
import {graphql, QueryRenderer} from 'react-relay';

import logo from './logo.svg';
import './App.css';

import environment from "./environment"

export default class App extends React.Component {
  render() {
    return (
      <QueryRenderer
        environment={environment}
        query={graphql`
          query AppQuery {
            getUserById(userId: 1) {
              username
            }  
          }
        `}
        variables={{}}
        render={({error, props}) => {
          if (error) {
            return <div>Error!</div>;
          }
          if (!this.props) {
            return <div>Loading...</div>;
          }
          return <div>User ID: {this.props.getUserById.username}</div>;
        }}
      />
    );
  }
}

// function App() {
//   return (
//     <div className="App">
//       <header className="App-header">
//         <img src={logo} className="App-logo" alt="logo" />
//         <p>
//           Edit <code>https://cms.budshome.com/graphql</code> and save to reload.
//         </p>
//         <a
//           className="App-link"
//           href="https://reactjs.org"
//           target="_blank"
//           rel="noopener noreferrer"
//         >
//           Learn React
//         </a>
//       </header>
//     </div>
//   );
// }

// export default App;
