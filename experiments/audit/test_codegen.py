"""Compiler-specific allocation clearing must not masquerade as erasure."""
import unittest
from check_codegen import check


class CodegenInitialization(unittest.TestCase):
    def test_zero_initialized_allocation_alone_does_not_prove_erasure(self):
        text = '''00000100 <audit_drop_packet>:
 100: call 200 <__rustc::__rdl_alloc_zeroed>
00000200 <__rustc::__rdl_alloc_zeroed>:
 200: xor %esi,%esi
 202: call 300 <memset>
'''
        with self.assertRaisesRegex(AssertionError, 'no recognized zero memset'):
            check(text, roots=('audit_drop_packet',))

    def test_real_clear_after_allocation_remains_required_and_recognized(self):
        text = '''00000100 <audit_drop_packet>:
 100: call 200 <__rustc::__rdl_alloc_zeroed>
 104: call 400 <production_clear>
00000200 <__rustc::__rdl_alloc_zeroed>:
 200: xor %esi,%esi
 202: call 300 <memset>
00000400 <production_clear>:
 400: xor %esi,%esi
 402: call 300 <memset>
'''
        result = check(text, roots=('audit_drop_packet',))
        self.assertEqual({hit['function'] for hit in result['audit_drop_packet']}, {'production_clear'})
