// Playground.tsx
// CruxLang Playground: Main interactive code editor and runner for the web frontend.
// Provides Monaco Editor integration, file management, code execution, and documentation panel.

import { useState } from 'react';
import MonacoEditor from '@monaco-editor/react';
import { FaFolder, FaCog, FaPlay, FaPlus, FaBook } from 'react-icons/fa';
import viteLogo from '/vite.svg';
import './App.css';

const DEFAULT_CODE = `emit "Hello, CruxLang!";`;

export default function Playground() {
  // === File State Management ===
  // These variables keep track of all the files the user is working on, which one is open,
  // and what their contents are. Think of it as a mini file system inside the browser.
  const [files, setFiles] = useState([
    { name: 'main.crx', code: DEFAULT_CODE }
  ]);
  const [activeFile, setActiveFile] = useState(0);
  const [output, setOutput] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);
  const [showDocs, setShowDocs] = useState(false);

  // === Code Execution Handler ===
  // When the user clicks 'Run', this function sends the code to the backend server.
  // It waits for the result and then updates the output or error message shown to the user.
  // This is like sending your code to a teacher and waiting for feedback.
  const runCode = async () => {
    setLoading(true);
    setOutput('');
    setError('');
    try {
      const response = await fetch('http://localhost:3000/run', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ code: files[activeFile].code }),
      });
      const data = await response.json();
      if (data.error) setError(data.error);
      else setOutput(data.output);
    } catch (e) {
      setError('Failed to connect to backend.');
    } finally {
      setLoading(false);
    }
  };

  // === File Explorer Sidebar ===
  // This section draws the sidebar where users can see all their files and switch between them.
  // It's like the file explorer in VSCode or any IDE.
  const sidebar = (
    <div className="playground-sidebar">
      <div className="sidebar-header">
        <span><FaFolder /> Files</span>
        <button className="sidebar-add-btn" title="Add file"><FaPlus /></button>
      </div>
      <div className="sidebar-files">
        {files.map((file, idx) => (
          <div
            key={file.name}
            className={`sidebar-file${idx === activeFile ? ' active' : ''}`}
            onClick={() => setActiveFile(idx)}
          >
            {file.name}
          </div>
        ))}
      </div>
    </div>
  );

  // === Tab Bar ===
  // This section draws the tabs for each open file, letting users switch between them easily.
  // It's like browser tabs, but for code files.
  const tabBar = (
    <div className="playground-tabs">
      {files.map((file, idx) => (
        <div
          key={file.name}
          className={`playground-tab${idx === activeFile ? ' active' : ''}`}
          onClick={() => setActiveFile(idx)}
        >
          {file.name}
        </div>
      ))}
      <button className="tab-settings-btn" title="Settings"><FaCog /></button>
    </div>
  );

  // === Documentation Panel ===
  // This section shows the built-in documentation and language reference.
  // It's like having a quick reference guide right inside the playground.
  const Documentation = () => (
    <div className="docs-container">
      <div className="docs-header">
        <h1>CruxLang Syntax Reference</h1>
        <button className="docs-close-btn" onClick={() => setShowDocs(false)}>×</button>
      </div>
      <div className="docs-content">
        <section>
          <h2>Overview</h2>
          <p>CruxLang is a modern programming language with a clean, readable syntax. It supports imperative programming, object-oriented features, and functional programming constructs.</p>
        </section>

        <section>
          <h2>Basic Syntax</h2>
          <h3>Comments</h3>
          <pre><code>{`// This is a single-line comment`}</code></pre>
          <h3>Statements</h3>
          <p>All statements must end with a semicolon (;)</p>
          <pre><code>{`let x = 5;
emit "Hello World";`}</code></pre>
        </section>

        <section>
          <h2>Variables and Data Types</h2>
          <h3>Variable Declaration</h3>
          <pre><code>{`let variable_name = value;`}</code></pre>
          <h3>Data Types</h3>
          <ul>
            <li><strong>Numbers:</strong> <code>42</code>, <code>3.14</code></li>
            <li><strong>Strings:</strong> <code>"Hello World"</code></li>
            <li><strong>Booleans:</strong> <code>true</code>, <code>false</code></li>
            <li><strong>Arrays:</strong> <code>[1, 2, 3]</code></li>
          </ul>
          <h3>Array Access</h3>
          <pre><code>{`let arr = [10, 20, 30];
let first = arr[0];  // Access first element`}</code></pre>
        </section>

        <section>
          <h2>Output</h2>
          <p>Use the <code>emit</code> statement to output values:</p>
          <pre><code>{`emit "Hello World";
emit 42;
emit true;`}</code></pre>
        </section>

        <section>
          <h2>Control Flow</h2>
          <h3>If Statements</h3>
          <pre><code>{`if (condition) {
    // code here
} else {
    // code here
}`}</code></pre>
          <h3>While Loops</h3>
          <pre><code>{`while (condition) {
    // code here
}`}</code></pre>
          <h3>For Loops</h3>
          <pre><code>{`for variable in iterable {
    // code here
}`}</code></pre>
          <h3>Try-Catch Blocks</h3>
          <pre><code>{`try {
    // code that might throw
} catch {
    // error handling
}`}</code></pre>
        </section>

        <section>
          <h2>Functions</h2>
          <h3>Function Declaration</h3>
          <pre><code>{`func function_name(param1, param2) {
    // function body
    return value;
}`}</code></pre>
          <h3>Function Call</h3>
          <pre><code>{`let result = function_name(arg1, arg2);`}</code></pre>
          <h3>Return Statement</h3>
          <pre><code>{`return expression;`}</code></pre>
        </section>

        <section>
          <h2>Object-Oriented Programming</h2>
          <h3>Classes</h3>
          <pre><code>{`class ClassName {
    func method_name(param) {
        // method body
        return value;
    }
}`}</code></pre>
          <h3>Entities (Advanced Classes)</h3>
          <pre><code>{`entity EntityName {
    def method_name(param) {
        // method body
        return value;
    }
}`}</code></pre>
          <h3>Object Instantiation</h3>
          <pre><code>{`let obj = ClassName();`}</code></pre>
          <h3>Method Calls</h3>
          <pre><code>{`obj.method_name(arg);`}</code></pre>
          <h3>Field Access</h3>
          <pre><code>{`obj.field_name`}</code></pre>
        </section>

        <section>
          <h2>Operators</h2>
          <h3>Arithmetic Operators</h3>
          <ul>
            <li><code>+</code> Addition</li>
            <li><code>-</code> Subtraction</li>
            <li><code>*</code> Multiplication</li>
            <li><code>/</code> Division</li>
            <li><code>%</code> Modulo</li>
          </ul>
          <h3>Comparison Operators</h3>
          <ul>
            <li><code>==</code> Equal to</li>
            <li><code>!=</code> Not equal to</li>
            <li><code>{'<'}</code> Less than</li>
            <li><code>{'<='}</code> Less than or equal</li>
            <li><code>{'>'}</code> Greater than</li>
            <li><code>{'>='}</code> Greater than or equal</li>
          </ul>
          <h3>Logical Operators</h3>
          <ul>
            <li><code>and</code> Logical AND</li>
            <li><code>or</code> Logical OR</li>
            <li><code>not</code> Logical NOT</li>
          </ul>
          <h3>Assignment</h3>
          <pre><code>{`variable = value;`}</code></pre>
        </section>

        <section>
          <h2>Modules</h2>
          <h3>Module Import</h3>
          <pre><code>{`use module_name;`}</code></pre>
          <h3>Module Function Calls</h3>
          <pre><code>{`module_name::function_name(arg1, arg2);`}</code></pre>
        </section>

        <section>
          <h2>Special Keywords</h2>
          <ul>
            <li><code>self</code> - Reference to current object instance</li>
            <li><code>let</code> - Variable declaration</li>
            <li><code>func</code> / <code>function</code> - Function declaration</li>
            <li><code>class</code> - Class declaration</li>
            <li><code>entity</code> - Entity declaration</li>
            <li><code>def</code> - Method definition in entities</li>
            <li><code>return</code> - Return from function</li>
            <li><code>emit</code> - Output value</li>
            <li><code>if</code> / <code>else</code> - Conditional statements</li>
            <li><code>while</code> - While loop</li>
            <li><code>for</code> / <code>in</code> - For loop</li>
            <li><code>try</code> / <code>catch</code> - Exception handling</li>
            <li><code>use</code> - Module import</li>
          </ul>
        </section>

        <section>
          <h2>Examples</h2>
          <h3>Hello World</h3>
          <pre><code>{`emit "Hello, CruxLang!";`}</code></pre>
          <h3>Simple Function</h3>
          <pre><code>{`func factorial(n) {
    if n <= 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

let result = factorial(5);
emit result;`}</code></pre>
          <h3>Class with Methods</h3>
          <pre><code>{`class Calculator {
    func add(a, b) {
        return a + b;
    }
    
    func multiply(a, b) {
        return a * b;
    }
}

let calc = Calculator();
let sum = calc.add(5, 3);
emit sum;`}</code></pre>
          <h3>Array Operations</h3>
          <pre><code>{`let numbers = [1, 2, 3, 4, 5];
let sum = 0;

for i in numbers {
    sum = sum + i;
}

emit sum;`}</code></pre>
        </section>
      </div>
    </div>
  );

  return (
    <div className="playground-root">
      <header className="playground-header">
        <img src={viteLogo} className="playground-logo" alt="logo" />
        <h1>CruxLang Playground</h1>
        <button 
          className="docs-toggle-btn" 
          onClick={() => setShowDocs(!showDocs)}
          title="Toggle Documentation"
        >
          <FaBook />
        </button>
      </header>
      <div className="playground-main-flex">
        {sidebar}
        <div className="playground-main-content">
          {tabBar}
          <div className="playground-editor-output-flex">
            <div className="playground-editor-area">
              <MonacoEditor
                height="calc(100vh - 220px)"
                theme="vs-dark"
                language="javascript"
                value={files[activeFile].code}
                onChange={v => {
                  const newFiles = [...files];
                  newFiles[activeFile].code = v || '';
                  setFiles(newFiles);
                }}
                options={{ fontSize: 16, minimap: { enabled: false } }}
              />
              <button className="playground-run-btn" onClick={runCode} disabled={loading}>
                <FaPlay /> {loading ? 'Running...' : 'Run'}
              </button>
            </div>
            <div className="playground-output-area">
              <div className="playground-output-label">Output</div>
              <pre className="playground-output-box">{output}</pre>
              {error && <div className="playground-error">{error}</div>}
            </div>
          </div>
        </div>
      </div>
      {showDocs && <Documentation />}
      <footer className="playground-footer">
        CruxLang &copy; {new Date().getFullYear()} | Modern Playground
      </footer>
    </div>
  );
} 