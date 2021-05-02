# if !defined(LUMA_HDR_UTF8ENC)
# define LUMA_HDR_UTF8ENC
# include <stddef.h>
# include <stdint.h>
extern uint8_t const * luma_utf8enc(uint32_t * codep,size_t * outszptr);
# endif
