def fun(n):
    return (n - 2) ** 2 // 2 + 2

check = """1 1
2 2
3 4
4 6
5 10
6 14
7 20
8 26
9 34
10 42
11 52
12 62
13 74
14 86
15 100
16 114
17 130
18 146
19 164
20 182
21 202
22 222
23 244
24 266
25 290
26 314
27 340
28 366
29 394
30 422
31 452
32 482
33 514
34 546
35 580
36 614
37 650
38 686
39 724
40 762
41 802
42 842
43 884
44 926
45 970
46 1014
47 1060
48 1106
49 1154
50 1202
51 1252
52 1302
53 1354
54 1406
55 1460
56 1514
57 1570
58 1626
59 1684
60 1742
61 1802
62 1862
63 1924
64 1986
65 2050
66 2114
67 2180
68 2246
69 2314
70 2382
71 2452
72 2522
73 2594
74 2666
75 2740
76 2814
77 2890
78 2966
79 3044
80 3122
81 3202
82 3282
83 3364
84 3446
85 3530
86 3614
87 3700
88 3786
89 3874
90 3962
91 4052
92 4142
93 4234
94 4326
95 4420
96 4514
97 4610
98 4706
99 4804
100 4902
101 5002
102 5102
103 5204
104 5306
105 5410
106 5514
107 5620
108 5726
109 5834
110 5942
111 6052
112 6162
113 6274
114 6386
115 6500
116 6614
117 6730
118 6846
119 6964
120 7082
121 7202
122 7322
123 7444
124 7566
125 7690
126 7814
127 7940
128 8066
129 8194
130 8322
131 8452
132 8582
133 8714
134 8846
135 8980
136 9114
137 9250
138 9386
139 9524
140 9662
141 9802
142 9942
143 10084
144 10226
145 10370
146 10514
147 10660
148 10806
149 10954
150 11102
151 11252
152 11402
153 11554
154 11706
155 11860
156 12014
157 12170
158 12326
159 12484
160 12642
161 12802
162 12962
163 13124
164 13286
165 13450
166 13614
167 13780
168 13946
169 14114
170 14282
171 14452
172 14622
173 14794
174 14966
175 15140
176 15314
177 15490
178 15666
179 15844
180 16022
181 16202
182 16382
183 16564
184 16746
185 16930
186 17114
187 17300
188 17486
189 17674
190 17862
191 18052
192 18242
193 18434
194 18626
195 18820
196 19014
197 19210
198 19406
199 19604
200 19802
201 20002
202 20202
203 20404
204 20606
205 20810
206 21014
207 21220
208 21426
209 21634
210 21842
211 22052
212 22262
213 22474
214 22686
215 22900
216 23114
217 23330
218 23546
219 23764
220 23982
221 24202
222 24422
223 24644
224 24866
225 25090
226 25314
227 25540
228 25766
229 25994
230 26222
231 26452
232 26682
233 26914
234 27146
235 27380
236 27614
237 27850
238 28086
239 28324
240 28562
241 28802
242 29042
243 29284
244 29526
245 29770
246 30014
247 30260
248 30506
249 30754
250 31002
251 31252
252 31502
253 31754
254 32006
255 32260
256 32514
257 32770
258 33026
259 33284
260 33542
261 33802
262 34062
263 34324
264 34586
265 34850
266 35114
267 35380
268 35646
269 35914
270 36182
271 36452
272 36722
273 36994
274 37266
275 37540
276 37814
277 38090
278 38366
279 38644
280 38922
281 39202
282 39482
283 39764
284 40046
285 40330
286 40614
287 40900
288 41186
289 41474
290 41762
291 42052
292 42342
293 42634
294 42926
295 43220
296 43514
297 43810
298 44106
299 44404
300 44702
301 45002
302 45302
303 45604
304 45906
305 46210
306 46514
307 46820
308 47126
309 47434
310 47742
311 48052
312 48362
313 48674
314 48986
315 49300
316 49614
317 49930
318 50246
319 50564
320 50882
321 51202
322 51522
323 51844
324 52166
325 52490
326 52814
327 53140
328 53466
329 53794
330 54122
331 54452
332 54782
333 55114
334 55446
335 55780
336 56114
337 56450
338 56786
339 57124
340 57462
341 57802
342 58142
343 58484
344 58826
345 59170
346 59514
347 59860
348 60206
349 60554
350 60902
351 61252
352 61602
353 61954
354 62306
355 62660
356 63014
357 63370
358 63726
359 64084
360 64442
361 64802
362 65162
363 65524
364 65886
365 66250
366 66614
367 66980
368 67346
369 67714
370 68082
371 68452
372 68822
373 69194
374 69566
375 69940
376 70314
377 70690
378 71066
379 71444
380 71822
381 72202
382 72582
383 72964
384 73346
385 73730
386 74114
387 74500
388 74886
389 75274
390 75662
391 76052
392 76442
393 76834
394 77226
395 77620
396 78014
397 78410
398 78806
399 79204
400 79602
401 80002
402 80402
403 80804
404 81206
405 81610
406 82014
407 82420
408 82826
409 83234
410 83642
411 84052
412 84462
413 84874
414 85286
415 85700
416 86114
417 86530
418 86946
419 87364
420 87782
421 88202
422 88622
423 89044
424 89466
425 89890
426 90314
427 90740
428 91166
429 91594
430 92022
431 92452
432 92882
433 93314
434 93746
435 94180
436 94614
437 95050
438 95486
439 95924
440 96362
441 96802
442 97242
443 97684
444 98126
445 98570
446 99014
447 99460
448 99906
449 100354
450 100802
451 101252
452 101702
453 102154
454 102606
455 103060
456 103514
457 103970
458 104426
459 104884
460 105342
461 105802
462 106262
463 106724
464 107186
465 107650
466 108114
467 108580
468 109046
469 109514
470 109982
471 110452
472 110922
473 111394
474 111866
475 112340
476 112814
477 113290
478 113766
479 114244
480 114722
481 115202
482 115682
483 116164
484 116646
485 117130
486 117614
487 118100
488 118586
489 119074
490 119562
491 120052
492 120542
493 121034
494 121526
495 122020
496 122514
497 123010
498 123506
499 124004
500 124502
501 125002
502 125502
503 126004
504 126506
505 127010
506 127514
507 128020
508 128526
509 129034
510 129542
511 130052
512 130562
513 131074
514 131586
515 132100
516 132614
517 133130
518 133646
519 134164
520 134682
521 135202
522 135722
523 136244
524 136766
525 137290
526 137814
527 138340
528 138866
529 139394
530 139922
531 140452
532 140982
533 141514
534 142046
535 142580
536 143114
537 143650
538 144186
539 144724
540 145262
541 145802
542 146342
543 146884
544 147426
545 147970
546 148514
547 149060
548 149606
549 150154
550 150702
551 151252
552 151802
553 152354
554 152906
555 153460
556 154014
557 154570
558 155126
559 155684
560 156242
561 156802
562 157362
563 157924
564 158486
565 159050
566 159614
567 160180
568 160746
569 161314
570 161882
571 162452
572 163022
573 163594
574 164166
575 164740
576 165314
577 165890
578 166466
579 167044
580 167622
581 168202
582 168782
583 169364
584 169946
585 170530
586 171114
587 171700
588 172286
589 172874
590 173462
591 174052
592 174642
593 175234
594 175826
595 176420
596 177014
597 177610
598 178206
599 178804
600 179402
601 180002
602 180602
603 181204
604 181806
605 182410
606 183014
607 183620
608 184226
609 184834
610 185442
611 186052
612 186662
613 187274
614 187886
615 188500
616 189114
617 189730
618 190346
619 190964
620 191582
621 192202
622 192822
623 193444
624 194066
625 194690
626 195314
627 195940
628 196566
629 197194
630 197822
631 198452
632 199082
633 199714
634 200346
635 200980
636 201614
637 202250
638 202886
639 203524
640 204162
641 204802
642 205442
643 206084
644 206726
645 207370
646 208014
647 208660
648 209306
649 209954
650 210602
651 211252
652 211902
653 212554
654 213206
655 213860
656 214514
657 215170
658 215826
659 216484
660 217142
661 217802
662 218462
663 219124
664 219786
665 220450
666 221114
667 221780
668 222446
669 223114
670 223782
671 224452
672 225122
673 225794
674 226466
675 227140
676 227814
677 228490
678 229166
679 229844
680 230522
681 231202
682 231882
683 232564
684 233246
685 233930
686 234614
687 235300
688 235986
689 236674
690 237362
691 238052
692 238742
693 239434
694 240126
695 240820
696 241514
697 242210
698 242906
699 243604
700 244302
701 245002
702 245702
703 246404
704 247106
705 247810
706 248514
707 249220
708 249926
709 250634
710 251342
711 252052
712 252762
713 253474
714 254186
715 254900
716 255614
717 256330
718 257046
719 257764
720 258482
721 259202
722 259922
723 260644
724 261366
725 262090
726 262814
727 263540
728 264266
729 264994
730 265722
731 266452
732 267182
733 267914
734 268646
735 269380
736 270114
737 270850
738 271586
739 272324
740 273062
741 273802
742 274542
743 275284
744 276026
745 276770
746 277514
747 278260
748 279006
749 279754
750 280502
751 281252
752 282002
753 282754
754 283506
755 284260
756 285014
757 285770
758 286526
759 287284
760 288042
761 288802
762 289562
763 290324
764 291086
765 291850
766 292614
767 293380
768 294146
769 294914
770 295682
771 296452
772 297222
773 297994
774 298766
775 299540
776 300314
777 301090
778 301866
779 302644
780 303422
781 304202
782 304982
783 305764
784 306546
785 307330
786 308114
787 308900
788 309686
789 310474
790 311262
791 312052
792 312842
793 313634
794 314426
795 315220
796 316014
797 316810
798 317606
799 318404
800 319202
801 320002
802 320802
803 321604
804 322406
805 323210
806 324014
807 324820
808 325626
809 326434
810 327242
811 328052
812 328862
813 329674
814 330486
815 331300
816 332114
817 332930
818 333746
819 334564
820 335382
821 336202
822 337022
823 337844
824 338666
825 339490
826 340314
827 341140
828 341966
829 342794
830 343622
831 344452
832 345282
833 346114
834 346946
835 347780
836 348614
837 349450
838 350286
839 351124
840 351962
841 352802
842 353642
843 354484
844 355326
845 356170
846 357014
847 357860
848 358706
849 359554
850 360402
851 361252
852 362102
853 362954
854 363806
855 364660
856 365514
857 366370
858 367226
859 368084
860 368942
861 369802
862 370662
863 371524
864 372386
865 373250
866 374114
867 374980
868 375846
869 376714
870 377582
871 378452
872 379322
873 380194
874 381066
875 381940
876 382814
877 383690
878 384566
879 385444
880 386322
881 387202
882 388082
883 388964
884 389846
885 390730
886 391614
887 392500
888 393386
889 394274
890 395162
891 396052
892 396942
893 397834
894 398726
895 399620
896 400514
897 401410
898 402306
899 403204
900 404102
901 405002
902 405902
903 406804
904 407706
905 408610
906 409514
907 410420
908 411326
909 412234
910 413142
911 414052
912 414962
913 415874
914 416786
915 417700
916 418614
917 419530
918 420446
919 421364
920 422282
921 423202
922 424122
923 425044
924 425966
925 426890
926 427814
927 428740
928 429666
929 430594
930 431522
931 432452
932 433382
933 434314
934 435246
935 436180
936 437114
937 438050
938 438986
939 439924
940 440862
941 441802
942 442742
943 443684
944 444626
945 445570
946 446514
947 447460
948 448406
949 449354
950 450302
951 451252
952 452202
953 453154
954 454106
955 455060
956 456014
957 456970
958 457926
959 458884
960 459842
961 460802
962 461762
963 462724
964 463686
965 464650
966 465614
967 466580
968 467546
969 468514
970 469482
971 470452
972 471422
973 472394
974 473366
975 474340
976 475314
977 476290
978 477266
979 478244
980 479222
981 480202
982 481182
983 482164
984 483146
985 484130
986 485114
987 486100
988 487086
989 488074
990 489062
991 490052
992 491042
993 492034
994 493026
995 494020
996 495014
997 496010
998 497006
999 498004
1000 499002"""

check = check.split('\n')
for n in check:
    v = n.split()
    o = fun(int(v[0])+1)
    print(o, v[1])

print(out)