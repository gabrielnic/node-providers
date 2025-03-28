"use client";
import 'bootstrap/dist/css/bootstrap.min.css';

import { AccountData, GraphNode } from './types';
import { useEffect, useState } from "react";
import { useWindowSize } from './hooks/useWindowSize';
import { GraphContainer } from './GraphContainer';


export default function Home() {
 

  const [loading, setLoading] = useState<boolean>(true);
  const [data, setData] = useState<AccountData[]>([]);
  useEffect(() => {
    fetch("/account_transactions.json")
      .then((res) => res.json())
      .then((data) => {
        setData(data);
        setLoading(false);
      })
      .catch((err) => { console.error("Failed to load JSON:", err); setLoading(false); });
  }, []);

  const [selectedNode, setSelectedNode] = useState<GraphNode | null>(null);
  const handleNodeClick = (node: GraphNode) => {
    setSelectedNode(node);
  };


  const { width, height } = useWindowSize();
 
  return (
    <>
      <svg xmlns="http://www.w3.org/2000/svg" className="d-none">
        <symbol id="check2" viewBox="0 0 16 16">
          <path d="M13.854 3.646a.5.5 0 0 1 0 .708l-7 7a.5.5 0 0 1-.708 0l-3.5-3.5a.5.5 0 1 1 .708-.708L6.5 10.293l6.646-6.647a.5.5 0 0 1 .708 0z" />
        </symbol>
        <symbol id="circle-half" viewBox="0 0 16 16">
          <path d="M8 15A7 7 0 1 0 8 1v14zm0 1A8 8 0 1 1 8 0a8 8 0 0 1 0 16z" />
        </symbol>
        <symbol id="moon-stars-fill" viewBox="0 0 16 16">
          <path d="M6 .278a.768.768 0 0 1 .08.858 7.208 7.208 0 0 0-.878 3.46c0 4.021 3.278 7.277 7.318 7.277.527 0 1.04-.055 1.533-.16a.787.787 0 0 1 .81.316.733.733 0 0 1-.031.893A8.349 8.349 0 0 1 8.344 16C3.734 16 0 12.286 0 7.71 0 4.266 2.114 1.312 5.124.06A.752.752 0 0 1 6 .278z" />
          <path d="M10.794 3.148a.217.217 0 0 1 .412 0l.387 1.162c.173.518.579.924 1.097 1.097l1.162.387a.217.217 0 0 1 0 .412l-1.162.387a1.734 1.734 0 0 0-1.097 1.097l-.387 1.162a.217.217 0 0 1-.412 0l-.387-1.162A1.734 1.734 0 0 0 9.31 6.593l-1.162-.387a.217.217 0 0 1 0-.412l1.162-.387a1.734 1.734 0 0 0 1.097-1.097l.387-1.162zM13.863.099a.145.145 0 0 1 .274 0l.258.774c.115.346.386.617.732.732l.774.258a.145.145 0 0 1 0 .274l-.774.258a1.156 1.156 0 0 0-.732.732l-.258.774a.145.145 0 0 1-.274 0l-.258-.774a1.156 1.156 0 0 0-.732-.732l-.774-.258a.145.145 0 0 1 0-.274l.774-.258c.346-.115.617-.386.732-.732L13.863.1z" />
        </symbol>
        <symbol id="sun-fill" viewBox="0 0 16 16">
          <path d="M8 12a4 4 0 1 0 0-8 4 4 0 0 0 0 8zM8 0a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 0zm0 13a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 13zm8-5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2a.5.5 0 0 1 .5.5zM3 8a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2A.5.5 0 0 1 3 8zm10.657-5.657a.5.5 0 0 1 0 .707l-1.414 1.415a.5.5 0 1 1-.707-.708l1.414-1.414a.5.5 0 0 1 .707 0zm-9.193 9.193a.5.5 0 0 1 0 .707L3.05 13.657a.5.5 0 0 1-.707-.707l1.414-1.414a.5.5 0 0 1 .707 0zm9.193 2.121a.5.5 0 0 1-.707 0l-1.414-1.414a.5.5 0 0 1 .707-.707l1.414 1.414a.5.5 0 0 1 0 .707zM4.464 4.465a.5.5 0 0 1-.707 0L2.343 3.05a.5.5 0 1 1 .707-.707l1.414 1.414a.5.5 0 0 1 0 .708z" />
        </symbol>
      </svg>

      <div className="dropdown position-fixed bottom-0 end-0 mb-3 me-3 bd-mode-toggle">
    
        <button
          className="btn btn-bd-primary py-2 dropdown-toggle d-flex align-items-center"
          id="bd-theme"
          type="button"
          aria-expanded="false"
          data-bs-toggle="dropdown"
          aria-label="Toggle theme (auto)"
        >
          <svg className="bi my-1 theme-icon-active" width="1em" height="1em">
            <use href="#circle-half" />
          </svg>
          <span className="visually-hidden" id="bd-theme-text">
            Toggle theme
          </span>
        </button>
        <ul className="dropdown-menu dropdown-menu-end shadow" aria-labelledby="bd-theme-text">
          <li>
            <button
              type="button"
              className="dropdown-item d-flex align-items-center"
              data-bs-theme-value="light"
              aria-pressed="false"
            >
              <svg className="bi me-2 opacity-50" width="1em" height="1em">
                <use href="#sun-fill" />
              </svg>
              Light
              <svg className="bi ms-auto d-none" width="1em" height="1em">
                <use href="#check2" />
              </svg>
            </button>
          </li>
          <li>
            <button
              type="button"
              className="dropdown-item d-flex align-items-center"
              data-bs-theme-value="dark"
              aria-pressed="false"
            >
              <svg className="bi me-2 opacity-50" width="1em" height="1em">
                <use href="#moon-stars-fill" />
              </svg>
              Dark
              <svg className="bi ms-auto d-none" width="1em" height="1em">
                <use href="#check2" />
              </svg>
            </button>
          </li>
          <li>
            <button
              type="button"
              className="dropdown-item d-flex align-items-center active"
              data-bs-theme-value="auto"
              aria-pressed="true"
            >
              <svg className="bi me-2 opacity-50" width="1em" height="1em">
                <use href="#circle-half" />
              </svg>
              Auto
              <svg className="bi ms-auto d-none" width="1em" height="1em">
                <use href="#check2" />
              </svg>
            </button>
          </li>
        </ul>
      </div>

      <svg xmlns="http://www.w3.org/2000/svg" className="d-none">
        <symbol
          id="aperture"
          fill="none"
          stroke="currentColor"
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth="2"
          viewBox="0 0 24 24"
        >
          <circle cx="12" cy="12" r="10" />
          <path d="M14.31 8l5.74 9.94M9.69 8h11.48M7.38 12l5.74-9.94M9.69 16L3.95 6.06M14.31 16H2.83m13.79-4l-5.74 9.94" />
        </symbol>
        <symbol id="cart" viewBox="0 0 16 16">
          <path d="M0 1.5A.5.5 0 0 1 .5 1H2a.5.5 0 0 1 .485.379L2.89 3H14.5a.5.5 0 0 1 .49.598l-1 5a.5.5 0 0 1-.465.401l-9.397.472L4.415 11H13a.5.5 0 0 1 0 1H4a.5.5 0 0 1-.491-.408L2.01 3.607 1.61 2H.5a.5.5 0 0 1-.5-.5zM3.102 4l.84 4.479 9.144-.459L13.89 4H3.102zM5 12a2 2 0 1 0 0 4 2 2 0 0 0 0-4zm7 0a2 2 0 1 0 0 4 2 2 0 0 0 0-4zm-7 1a1 1 0 1 1 0 2 1 1 0 0 1 0-2zm7 0a1 1 0 1 1 0 2 1 1 0 0 1 0-2z" />
        </symbol>
        <symbol id="chevron-right" viewBox="0 0 16 16">
          <path fillRule="evenodd" d="M4.646 1.646a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1 0 .708l-6 6a.5.5 0 0 1-.708-.708L10.293 8 4.646 2.354a.5.5 0 0 1 0-.708z" />
        </symbol>
      </svg>

      <nav className="navbar navbar-expand-md bg-dark sticky-top border-bottom" data-bs-theme="dark">
        <div className="container">
          <a className="navbar-brand d-md-none" href="#">
            <svg className="bi" width="24" height="24">
              <use xlinkHref="#aperture" />
            </svg>
            Aperture
          </a>
          <button
            className="navbar-toggler"
            type="button"
            data-bs-toggle="offcanvas"
            data-bs-target="#offcanvas"
            aria-controls="offcanvas"
            aria-label="Toggle navigation"
          >
            <span className="navbar-toggler-icon"></span>
          </button>
          <div className="offcanvas offcanvas-end" id="offcanvas" aria-labelledby="offcanvasLabel">
            <div className="offcanvas-header">
              <h5 className="offcanvas-title" id="offcanvasLabel">
                Aperture
              </h5>
              <button type="button" className="btn-close" data-bs-dismiss="offcanvas" aria-label="Close"></button>
            </div>
            <div className="offcanvas-body">
              <ul className="navbar-nav flex-grow-1 justify-content-between">
                <li className="nav-item">
                  <a className="nav-link" href="#">
                    <svg className="bi" width="24" height="24">
                      <use xlinkHref="#aperture" />
                    </svg>
                  </a>
                </li>
                {/* <li className="nav-item">
                <a className="nav-link" href="#">
                  Tour
                </a>
              </li> */}

                {/* <li className="nav-item">
                <a className="nav-link" href="#">
                  <svg className="bi" width="24" height="24">
                    <use xlinkHref="#cart" />
                  </svg>
                </a>
              </li> */}
              </ul>
            </div>
          </div>
        </div>
      </nav>

      <main>
        <div className="position-relative  overflow-hidden  m-md-3 text-center bg-body-tertiary">
          <div className="row align-items-center">
         
            <div className='col-3'>
            </div>
            <div className='col-3'>
              <div className="card">
                <div className="card-header">
                  <h5 className="card-title">Selected Node</h5>
                </div>
                <div className="card-body">
                  {selectedNode ? (
                    <div>
                      <p>
                        <strong>Label:</strong> {selectedNode.label}
                      </p>
                      <p className="account-id">
                        <strong>Account ID:</strong> {selectedNode.id}
                      </p>
                      <p>
                        <strong>Type:</strong> {selectedNode.group}
                      </p>
                    </div>
                  ) : (
                    <p>No node selected.</p>
                  )}
                </div>
              </div>
            </div>
          </div>
          <div className="product-device shadow-sm d-none d-md-block"></div>
          <div className="product-device product-device-2 shadow-sm d-none d-md-block"></div>
        </div>

        <div className="position-relative overflow-hidden  m-md-3 text-center bg-body-tertiary">
          <div className="container-fluid" style={{ height: "100vh" }}>
            <div className="row h-100">
              <div className="col-12 h-100 border">
                <GraphContainer
                  data={data}
                  width={width} // adjust as needed
                  height={height}
                  onNodeClick={handleNodeClick}
                  loading={loading}
                />
              </div>
            </div>
          </div>

        </div>


      </main>

      <footer className="container py-5">
        <div className="row">
          <div className="col-12 col-md">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              fill="none"
              stroke="currentColor"
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              className="d-block mb-2"
              role="img"
              viewBox="0 0 24 24"
            >
              <title>Product</title>
              <circle cx="12" cy="12" r="10" />
              <path d="M14.31 8l5.74 9.94M9.69 8h11.48M7.38 12l5.74-9.94M9.69 16L3.95 6.06M14.31 16H2.83m13.79-4l-5.74 9.94" />
            </svg>
            <small className="d-block mb-3 text-body-secondary">&copy; 2017–2024</small>
          </div>
          <div className="col-6 col-md">
            <h5>Features</h5>
            <ul className="list-unstyled text-small">
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Cool stuff
                </a>
              </li>
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Random feature
                </a>
              </li>
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Team feature
                </a>
              </li>
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Stuff for developers
                </a>
              </li>
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Another one
                </a>
              </li>
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Last time
                </a>
              </li>
            </ul>
          </div>
          <div className="col-6 col-md">
            <h5>Resources</h5>
            <ul className="list-unstyled text-small">
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Resource name
                </a>
              </li>
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Resource
                </a>
              </li>
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Another resource
                </a>
              </li>
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Final resource
                </a>
              </li>
            </ul>
          </div>
          <div className="col-6 col-md">
            <h5>Resources</h5>
            <ul className="list-unstyled text-small">
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Business
                </a>
              </li>
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Education
                </a>
              </li>
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Government
                </a>
              </li>
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Gaming
                </a>
              </li>
            </ul>
          </div>
          <div className="col-6 col-md">
            <h5>About</h5>
            <ul className="list-unstyled text-small">
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Team
                </a>
              </li>
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Locations
                </a>
              </li>
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Privacy
                </a>
              </li>
              <li>
                <a className="link-secondary text-decoration-none" href="#">
                  Terms
                </a>
              </li>
            </ul>
          </div>
        </div>
      </footer>
    </>
  )
}
