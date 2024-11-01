import { Html, Head, Main, NextScript } from "next/document";


const title = "Brick by Brick";
const description = "A habit tracker app built by with Next.js and Rust";
const url = "https://brick-by-brick.vercel.app";

export const metadata = {
  title,
  description,
  url,
};

export default function Document() {
  return (
    <Html lang="en">
      <Head />
      <body className="antialiased">
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}
