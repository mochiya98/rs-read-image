import { expect } from "chai";
import { readImage } from "..";
import Jimp from "jimp";

const DUMMY_PIXEL = [12, 34, 56, 78];

function createJimpAsync(arg0: {
  data: Buffer;
  width: number;
  height: number;
}) {
  return new Promise<Jimp>((resolve, reject) => {
    new Jimp(arg0, (err, image) => {
      if (err) reject(err);
      resolve(image);
    }).getBuffer;
  });
}

describe("readImage", function () {
  let dummyImg: Jimp;
  let dummyImgBuf: Buffer;

  before(async () => {
    dummyImg = await createJimpAsync({
      data: Buffer.from(DUMMY_PIXEL),
      width: 1,
      height: 1,
    });
    dummyImgBuf = await dummyImg.getBufferAsync("image/png");
  });

  it("successful", async function () {
    const result = readImage(dummyImgBuf);
    expect(Array.from(result)).to.eql(DUMMY_PIXEL);
  });
  it("failed", function () {
    //@ts-ignore
    expect(() => readImage()).to.throw("invalid_argument");
    expect(() => readImage([])).to.throw(
      "The image format could not be determined"
    );
  });
});
