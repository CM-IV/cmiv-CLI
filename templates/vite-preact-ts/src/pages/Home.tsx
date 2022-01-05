import { Layout } from "../layouts/layout";
import axios from "axios";
import { useEffect, useState } from "preact/hooks";

const Home = () => {
  const [postData, setPostData] = useState<Posts[]>([]);

  const getPosts = () => {
    axios.get("https://jsonplaceholder.typicode.com/posts")
      .then((posts) => {
        setPostData(posts.data);
      })
      .catch((err) => {
        console.error(err);
      });
  }

  useEffect(() => {
    getPosts();
  }, [])

  return (
    <Layout>
      <section class="section">
        <h1 class="title">Hello!</h1>
        <h2 class="subtitle">Here are some posts:</h2>
      </section>
      
      {postData.map((posts) => {
        return (
          <div class="box" key={posts.id}>
            <h3 class="subtitle">{posts.title}</h3>
            <p>{posts.body}</p>
          </div>
        )
      })}
    </Layout>
  )
}


export { Home };