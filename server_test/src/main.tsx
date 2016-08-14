import * as React from 'react';
import * as ReactDOM from 'react-dom';

class TestPanel extends React.Component<{}, {}> {
  render(): JSX.Element {
    return (
      <div>
        <button>haha</button>
      </div>
    );
  }
}

ReactDOM.render(
  <TestPanel />,
  document.getElementById('content')
);

