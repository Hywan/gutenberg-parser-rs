dnl $Id$
dnl config.m4 for extension gutenberg_post_parser

dnl Comments in this file start with the string 'dnl'.
dnl Remove where necessary.

dnl If your extension references something external, use with:

dnl PHP_ARG_WITH(gutenberg_post_parser, for gutenberg_post_parser support,
dnl Make sure that the comment is aligned:
dnl [  --with-gutenberg_post_parser             Include gutenberg_post_parser support])

dnl Otherwise use enable:

PHP_ARG_ENABLE(gutenberg_post_parser, whether to enable gutenberg_post_parser support,
[  --with-gutenberg_post_parser          Include gutenberg_post_parser support], no)

if test "$PHP_GUTENBERG_POST_PARSER" != "no"; then
  dnl Write more examples of tests here...

  dnl # get library FOO build options from pkg-config output
  dnl AC_PATH_PROG(PKG_CONFIG, pkg-config, no)
  dnl AC_MSG_CHECKING(for libfoo)
  dnl if test -x "$PKG_CONFIG" && $PKG_CONFIG --exists foo; then
  dnl   if $PKG_CONFIG foo --atleast-version 1.2.3; then
  dnl     LIBFOO_CFLAGS=\`$PKG_CONFIG foo --cflags\`
  dnl     LIBFOO_LIBDIR=\`$PKG_CONFIG foo --libs\`
  dnl     LIBFOO_VERSON=\`$PKG_CONFIG foo --modversion\`
  dnl     AC_MSG_RESULT(from pkgconfig: version $LIBFOO_VERSON)
  dnl   else
  dnl     AC_MSG_ERROR(system libfoo is too old: version 1.2.3 required)
  dnl   fi
  dnl else
  dnl   AC_MSG_ERROR(pkg-config not found)
  dnl fi
  dnl PHP_EVAL_LIBLINE($LIBFOO_LIBDIR, GUTENBERG_POST_PARSER_SHARED_LIBADD)
  dnl PHP_EVAL_INCLINE($LIBFOO_CFLAGS)

  dnl # --with-gutenberg_post_parser -> check with-path
  dnl SEARCH_PATH="/usr/local /usr"     # you might want to change this
  dnl SEARCH_FOR="/include/gutenberg_post_parser.h"  # you most likely want to change this
  dnl if test -r $PHP_GUTENBERG_POST_PARSER/$SEARCH_FOR; then # path given as parameter
  dnl   GUTENBERG_POST_PARSER_DIR=$PHP_GUTENBERG_POST_PARSER
  dnl else # search default path list
  dnl   AC_MSG_CHECKING([for gutenberg_post_parser files in default path])
  dnl   for i in $SEARCH_PATH ; do
  dnl     if test -r $i/$SEARCH_FOR; then
  dnl       GUTENBERG_POST_PARSER_DIR=$i
  dnl       AC_MSG_RESULT(found in $i)
  dnl     fi
  dnl   done
  dnl fi
  dnl
  dnl if test -z "$GUTENBERG_POST_PARSER_DIR"; then
  dnl   AC_MSG_RESULT([not found])
  dnl   AC_MSG_ERROR([Please reinstall the gutenberg_post_parser distribution])
  dnl fi

  dnl # --with-gutenberg_post_parser -> add include path
  dnl PHP_ADD_INCLUDE($GUTENBERG_POST_PARSER_DIR/include)

  dnl # --with-gutenberg_post_parser -> check for lib and symbol presence
  dnl LIBNAME=GUTENBERG_POST_PARSER # you may want to change this
  dnl LIBSYMBOL=GUTENBERG_POST_PARSER # you most likely want to change this 

  dnl PHP_CHECK_LIBRARY($LIBNAME,$LIBSYMBOL,
  dnl [
  dnl   PHP_ADD_LIBRARY_WITH_PATH($LIBNAME, $GUTENBERG_POST_PARSER_DIR/$PHP_LIBDIR, GUTENBERG_POST_PARSER_SHARED_LIBADD)
  dnl   AC_DEFINE(HAVE_GUTENBERG_POST_PARSERLIB,1,[ ])
  dnl ],[
  dnl   AC_MSG_ERROR([wrong gutenberg_post_parser lib version or lib not found])
  dnl ],[
  dnl   -L$GUTENBERG_POST_PARSER_DIR/$PHP_LIBDIR -lm
  dnl ])
  dnl
  dnl PHP_SUBST(GUTENBERG_POST_PARSER_SHARED_LIBADD)

  dnl # In case of no dependencies
  dnl AC_DEFINE(HAVE_GUTENBERG_POST_PARSER, 1, [ Have gutenberg_post_parser support ])

  PHP_SUBST(GUTENBERG_POST_PARSER_SHARED_LIBADD)

  PHP_ADD_LIBRARY_WITH_PATH(gutenberg_post_parser, ., GUTENBERG_POST_PARSER_SHARED_LIBADD)

  PHP_NEW_EXTENSION(gutenberg_post_parser, gutenberg_post_parser.c, $ext_shared)
fi
