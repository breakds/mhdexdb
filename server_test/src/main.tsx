import * as React from 'react'
import * as ReactDOM from 'react-dom'

async function getResponse() {
  try {
    let response = await fetch('http://localhost:12345');
    console.log(response);
    let result = await response.json();
    console.log(result);
  } catch (err) {
    console.log(err);
  }
}

class TestPanel extends React.Component<{}, {}> {
  render(): JSX.Element {
    return (
      <div style={{margin: "50%"}}>
        <button onClick={() => {
            getResponse();
          }}>
          haha
        </button>
      </div>
    );
  }
}

ReactDOM.render(
  <TestPanel />,
  document.getElementById('content')
);

