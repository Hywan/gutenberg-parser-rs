%token comment_opening <!--[ \n\r\t]*/?wp: -> block
%token phrase (?s).+?(?=(<!--))
%token wp_closing /wp:
%token block:comment_auto_closing /--> -> default
%token block:comment_closing --> -> default
%token block:name ([a-z0-9\-]+/)?[a-z0-9\-]+
%token block:attributes \{[^\}]+\}
%token block:ws [ \n\r\t]+
%token tail (?s).+

#block_list:
    ( block() | phrase() )* tail()?

#block:
    block_balanced() | block_void()

#block_balanced:
    ::comment_opening:: <name[0]> ::ws:: block_attributes()? ::ws::? ::comment_closing::
    block_list()
    ::comment_opening:: ::name[0]:: ::ws::? ::comment_closing::

#block_void:
    ::comment_opening:: <name> ::ws:: block_attributes()? ::ws::? ::comment_auto_closing::

#block_attributes:
    <attributes>

#phrase:
    <phrase>

tail:
    <tail> #phrase_tail
