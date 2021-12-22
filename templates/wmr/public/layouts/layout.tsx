import { Wrapper } from "../components/wrapper";
import { Nav } from "../components/nav";
import { Footer } from "../components/footer";

const Layout = (props: any) => {
  return (
    <Wrapper>
      <Nav />
      {props.children}
      <Footer />
    </Wrapper>
  );
};

export { Layout };
