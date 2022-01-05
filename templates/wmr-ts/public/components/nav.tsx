import { Fragment } from "preact";

const Nav = () => {
  return (
    <Fragment>
      <nav
        class="navbar"
        id="nav"
        role="navigation"
        aria-label="main navigation"
      >
        <div class="navbar-brand">
          <a class="image is-48x48 mt-1" href="/">
            <img
              src="https://ik.imagekit.io/xbkhabiqcy9/img/Occident_Tech_logo_wyOOeI6Bhcn.png?updatedAt=1637081403933"
              height="48"
              width="48"
              class="image-responsive"
              alt="logo"
            />
          </a>
          <a
            role="button"
            class="navbar-burger mt-2"
            aria-label="menu"
            aria-expanded="false"
            data-target="navMenu"
          >
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
          </a>
        </div>

        <div id="navMenu" class="navbar-menu is-mobile">
          <div class="navbar-start">
            <a href="/" class="navbar-item">
              Home
            </a>
            <a href="/about" class="navbar-item">
              About
            </a>
          </div>
        </div>
      </nav>
    </Fragment>
  );
};

export { Nav };
