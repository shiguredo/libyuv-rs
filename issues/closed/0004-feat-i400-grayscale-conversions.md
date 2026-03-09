# I400 グレースケール変換追加

## 概要

I400 (グレースケール / Y のみ) フォーマットの変換関数を追加する。

## 対象関数

- I400ToARGB
- I400ToI400
- I400ToI420
- I400ToNV21
- I400Copy
- I400Mirror
- I420ToI400
- ARGBToI400
- ARGBToG

## 解決方法

convert.rs に以下の 7 関数を追加した:

- i400_to_argb, i400_to_i400, i400_to_i420, i400_to_nv21
- i420_to_i400
- argb_to_i400, argb_to_g

I400Copy, I400Mirror は単純なプレーン操作で planar.rs の copy_plane, mirror_plane で代替可能。
I400ToARGBMatrix は issue 0016 に移譲した。
